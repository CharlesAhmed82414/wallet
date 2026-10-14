use std::sync::Arc;

use cacher::{SwapVaultAddressCacher, SwapVaultAddressKind};
use gem_client::{ClientError, ReqwestClient, Response};
use gem_tracing::error_with_fields;
use primitives::SwapProvider;
use reqwest::{Method, RequestBuilder};
use serde::Serialize;

pub(super) struct SwapProxyClient {
    client: ReqwestClient,
    deposit_addresses: Arc<dyn SwapVaultAddressCacher>,
    provider: SwapProvider,
    deposit_id_paths: &'static [&'static str],
}

impl SwapProxyClient {
    pub(super) fn new(url: String, deposit_addresses: Arc<dyn SwapVaultAddressCacher>, provider: SwapProvider, deposit_id_paths: &'static [&'static str]) -> Self {
        Self {
            client: ReqwestClient::new(url, gem_client::reqwest_client()),
            deposit_addresses,
            provider,
            deposit_id_paths,
        }
    }

    pub(super) async fn get<Q: Serialize + ?Sized>(&self, path: &str, query: &Q) -> Result<Response, ClientError> {
        self.send(self.client.request(Method::GET, path).query(query)).await
    }

    pub(super) async fn post<T: Serialize + ?Sized>(&self, path: &str, body: &T) -> Result<Response, ClientError> {
        self.send(self.client.request(Method::POST, path).json(body)).await
    }

    async fn send(&self, request: RequestBuilder) -> Result<Response, ClientError> {
        let response = self.client.send(request).await?;
        self.cache_deposit_ids(&response.data).await;
        Ok(response)
    }

    async fn cache_deposit_ids(&self, data: &[u8]) {
        let Ok(response) = serde_json::from_slice::<serde_json::Value>(data) else {
            return;
        };
        let deposit_ids = deposit_ids(&response, self.deposit_id_paths);
        if let Err(error) = self.deposit_addresses.add_vault_addresses(self.provider, SwapVaultAddressKind::Deposit, &deposit_ids).await {
            error_with_fields!("swap deposit address cache failed", &*error, provider = self.provider.as_ref());
        }
    }
}

fn deposit_ids(response: &serde_json::Value, paths: &[&str]) -> Vec<String> {
    paths.iter().filter_map(|path| response.pointer(path)?.as_str()).filter(|value| !value.is_empty()).map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_ids() {
        let paths = ["/steps/0/depositAddress", "/protocol/v2/orderId"];
        let bitcoin: serde_json::Value = serde_json::from_str(r#"{"steps":[{"id":"deposit","depositAddress":"bc1qa0z550eacdsxytnxf9yk7xvvhqzqvlx2tl44z2"}]}"#).unwrap();
        let evm: serde_json::Value = serde_json::from_str(r#"{"steps":[{"id":"deposit","depositAddress":""}],"protocol":{"v2":{"orderId":"0x5018b8125a7aec15f5a9e8bf07786e79cc1b52b6d97fb1a92930fc5aeea6ddf0"}}}"#).unwrap();
        let error: serde_json::Value = serde_json::from_str(r#"{"message":"Amount is too low","errorCode":"AMOUNT_TOO_LOW"}"#).unwrap();

        assert_eq!(deposit_ids(&bitcoin, &paths), vec!["bc1qa0z550eacdsxytnxf9yk7xvvhqzqvlx2tl44z2"]);
        assert_eq!(deposit_ids(&evm, &paths), vec!["0x5018b8125a7aec15f5a9e8bf07786e79cc1b52b6d97fb1a92930fc5aeea6ddf0"]);
        assert_eq!(deposit_ids(&error, &paths), Vec::<String>::new());

        let bridgers: serde_json::Value = serde_json::from_str(include_str!("../../../swapper/src/bridgers/testdata/swap_base_usdc_bsc_usdt.json")).unwrap();
        assert_eq!(
            deposit_ids(&bridgers, &["/data/txData/data"]),
            vec![
                "0x9ddf93bb000000000000000000000000833589fcd6edb6e08f4c7c32d4f71b54bda0291300000000000000000000000000000000000000000000000000000000000000a000000000000000000000000000000000000000000000000000000000000001000000000000000000000000000000000000000000000000000000000005f5e10000000000000000000000000000000000000000000000000560de40b7fdcf000000000000000000000000000000000000000000000000000000000000000000215553445428425343297c6874367a75747c302e3030357c62726964676572737c3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002a30783531344243623146394141626239303465363130364264313035324236366432373036644262623700000000000000000000000000000000000000000000"
            ]
        );
    }
}
