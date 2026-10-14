use gem_jsonrpc::types::ToJsonRpcRequest;
use serde_json::{Value, json};

use super::model::{ChainflipAsset, DcaParameters, DepositChannelRequest, VaultSwapExtras};

const REQUEST_SWAP_PARAMETER_ENCODING: &str = "broker_request_swap_parameter_encoding";
const REQUEST_SWAP_DEPOSIT_ADDRESS: &str = "broker_request_swap_deposit_address";

#[derive(Clone, Debug)]
pub(super) struct RequestSwapParameterEncoding {
    pub source_asset: ChainflipAsset,
    pub destination_asset: ChainflipAsset,
    pub destination_address: String,
    pub broker_commission: u32,
    pub extra_params: VaultSwapExtras,
    pub boost_fee: Option<u32>,
    pub dca_params: Option<DcaParameters>,
}

impl ToJsonRpcRequest for RequestSwapParameterEncoding {
    fn method(&self) -> &'static str {
        REQUEST_SWAP_PARAMETER_ENCODING
    }

    fn params(&self) -> Value {
        json!([
            self.source_asset,
            self.destination_asset,
            self.destination_address,
            self.broker_commission,
            self.extra_params,
            Value::Null,
            self.boost_fee,
            Vec::<Value>::new(),
            self.dca_params,
        ])
    }
}

impl ToJsonRpcRequest for DepositChannelRequest {
    fn method(&self) -> &'static str {
        REQUEST_SWAP_DEPOSIT_ADDRESS
    }

    fn params(&self) -> Value {
        json!([
            self.source_asset,
            self.destination_asset,
            self.destination_address,
            self.broker_commission,
            Value::Null,
            self.boost_fee,
            Vec::<Value>::new(),
            self.refund_parameters,
            self.dca_parameters,
        ])
    }
}

#[cfg(test)]
mod tests {
    use num_bigint::BigUint;

    use super::*;
    use crate::chainflip::broker::model::{RefundParameters, VaultSwapChainExtras};

    #[test]
    fn test_deposit_channel_params() {
        let request = DepositChannelRequest {
            source_asset: ChainflipAsset {
                chain: "Assethub".into(),
                asset: "DOT".into(),
            },
            destination_asset: ChainflipAsset {
                chain: "Ethereum".into(),
                asset: "ETH".into(),
            },
            destination_address: "destination".into(),
            broker_commission: 45,
            boost_fee: None,
            refund_parameters: RefundParameters {
                retry_duration: 150,
                refund_address: "refund".into(),
                min_price: "0x1".into(),
                max_oracle_price_slippage: None,
            },
            dca_parameters: Some(DcaParameters { number_of_chunks: 2, chunk_interval: 3 }),
        }
        .to_jsonrpc_request(1);

        assert_eq!(request.method, REQUEST_SWAP_DEPOSIT_ADDRESS);
        assert_eq!(
            request.params,
            json!([
                {"chain": "Assethub", "asset": "DOT"},
                {"chain": "Ethereum", "asset": "ETH"},
                "destination",
                45,
                null,
                null,
                [],
                {"retry_duration": 150, "refund_address": "refund", "min_price": "0x1"},
                {"number_of_chunks": 2, "chunk_interval": 3}
            ])
        );
    }

    #[test]
    fn test_params() {
        let extras = VaultSwapExtras::Evm(VaultSwapChainExtras {
            chain: "Ethereum".into(),
            input_amount: BigUint::from(1000u32),
            refund_parameters: RefundParameters::default(),
        });
        let request = RequestSwapParameterEncoding {
            source_asset: ChainflipAsset {
                chain: "Ethereum".into(),
                asset: "ETH".into(),
            },
            destination_asset: ChainflipAsset { chain: "Solana".into(), asset: "SOL".into() },
            destination_address: "destination".into(),
            broker_commission: 10,
            extra_params: extras.clone(),
            boost_fee: Some(5),
            dca_params: None,
        }
        .to_jsonrpc_request(7);

        assert_eq!(request.id, 7);
        assert_eq!(request.method, REQUEST_SWAP_PARAMETER_ENCODING);
        assert_eq!(
            request.params,
            json!([
                {"chain": "Ethereum", "asset": "ETH"},
                {"chain": "Solana", "asset": "SOL"},
                "destination",
                10,
                serde_json::to_value(&extras).unwrap(),
                null,
                5,
                [],
                null
            ])
        );
    }
}
