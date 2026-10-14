use std::{error::Error, sync::Arc};

use cacher::SwapVaultAddressCacher;
use gem_jsonrpc::types::{JsonRpcResult, ToJsonRpcRequest};
use primitives::SwapProvider;
use swapper::chainflip::broker::{DepositChannelRequest, DepositChannelResponse};
use swapper::chainflip::chain::ChainflipChain;
use swapper::fees::DEFAULT_CHAINFLIP_FEE_BPS;

use super::proxy_client::SwapProxyClient;

pub struct ChainflipProxyClient {
    client: SwapProxyClient,
}

impl ChainflipProxyClient {
    pub fn new(url: String, deposit_addresses: Arc<dyn SwapVaultAddressCacher>) -> Self {
        Self {
            client: SwapProxyClient::new(url, deposit_addresses, SwapProvider::Chainflip, &[]),
        }
    }

    pub async fn deposit_channel(&self, request: &DepositChannelRequest) -> Result<gem_client::Response, Box<dyn Error + Send + Sync>> {
        let request = broker_request(request)?;
        let response = self.client.post("", &request.to_jsonrpc_request(1)).await?;
        if let Some(deposit_address) = deposit_address(&response.data) {
            self.client.cache_deposit_id(deposit_address).await;
        }
        Ok(response)
    }
}

fn broker_request(request: &DepositChannelRequest) -> Result<DepositChannelRequest, Box<dyn Error + Send + Sync>> {
    if request.source_asset.chain != ChainflipChain::Assethub.as_ref() {
        return Err(format!("unsupported Chainflip deposit channel source: {}", request.source_asset.chain).into());
    }
    Ok(DepositChannelRequest {
        broker_commission: DEFAULT_CHAINFLIP_FEE_BPS,
        ..request.clone()
    })
}

fn deposit_address(data: &[u8]) -> Option<String> {
    serde_json::from_slice::<JsonRpcResult<DepositChannelResponse>>(data).ok()?.take().ok()?.deposit_address().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use swapper::chainflip::broker::{ChainflipAsset, RefundParameters};

    #[test]
    fn test_broker_request() {
        let request = DepositChannelRequest {
            source_asset: ChainflipAsset {
                chain: "Assethub".into(),
                asset: "DOT".into(),
            },
            destination_asset: ChainflipAsset {
                chain: "Ethereum".into(),
                asset: "ETH".into(),
            },
            destination_address: "0x2578f4fa0f36138c76389095bf74331a4f57bcc0".into(),
            broker_commission: 0,
            boost_fee: None,
            refund_parameters: RefundParameters::default(),
            dca_parameters: None,
        };

        assert_eq!(broker_request(&request).unwrap().broker_commission, DEFAULT_CHAINFLIP_FEE_BPS);

        let ethereum = DepositChannelRequest {
            source_asset: request.destination_asset.clone(),
            ..request
        };
        assert!(broker_request(&ethereum).is_err());
    }

    #[test]
    fn test_deposit_address() {
        let channel = br#"{"jsonrpc":"2.0","id":1,"result":{"address":"0xb72845b75b44e7e2ee29a390e9cf2e71291f2a1823299eedf5d016662fdb96cf","issued_block":15237919,"channel_id":23}}"#;
        let error = br#"{"jsonrpc":"2.0","id":1,"error":{"code":-32603,"message":"Invalid destination address"}}"#;

        assert_eq!(deposit_address(channel).as_deref(), Some("1599j8DFFZutdkvmxe81QNCpjNWAmmLD5pHSYph6gPVbp6av"));
        assert_eq!(deposit_address(error), None);
    }
}
