use serde::Deserialize;

use super::path::{PathRule, path_without_query};
use crate::jsonrpc_types::{JsonRpcRequest, RequestType};

pub const RPC_ROUTE: &str = "rpc";
const UNLISTED_ROUTE: &str = "unlisted";
pub const REJECTED_ROUTE: &str = "rejected";

#[derive(Debug, Default, Clone, Deserialize)]
#[serde(transparent)]
pub struct AllowlistConfig(Vec<AllowlistRule>);

impl AllowlistConfig {
    pub(super) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn route(&self, request_type: &RequestType) -> Option<String> {
        if self.0.is_empty() {
            return Some(unlisted_route(request_type));
        }

        match request_type {
            RequestType::Regular { path, method, .. } => self.0.iter().find_map(|rule| rule.path_route(path, method)),
            RequestType::JsonRpc(JsonRpcRequest::Single(call)) => self.0.iter().any(|rule| rule.matches_rpc(&call.method)).then(|| RPC_ROUTE.to_string()),
            RequestType::JsonRpc(JsonRpcRequest::Batch(calls)) => calls.iter().all(|call| self.0.iter().any(|rule| rule.matches_rpc(&call.method))).then(|| RPC_ROUTE.to_string()),
        }
    }
}

pub(super) fn unlisted_route(request_type: &RequestType) -> String {
    match request_type {
        RequestType::Regular { .. } => UNLISTED_ROUTE.to_string(),
        RequestType::JsonRpc(_) => RPC_ROUTE.to_string(),
    }
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum AllowlistRule {
    Rpc { rpc_method: String },
    Path(PathRule),
}

impl AllowlistRule {
    fn path_route(&self, path: &str, method: &str) -> Option<String> {
        match self {
            Self::Path(rule) => rule.matches(method, path_without_query(path)).then(|| rule.path.clone()),
            Self::Rpc { .. } => None,
        }
    }

    fn matches_rpc(&self, rpc_method: &str) -> bool {
        match self {
            Self::Rpc { rpc_method: allowed } => allowed == rpc_method,
            Self::Path(_) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use config::{Config, File, FileFormat};
    use serde_json::json;

    use super::*;

    #[test]
    fn test_allows_jsonrpc_single_method() {
        let config = AllowlistConfig::mock();

        assert_eq!(config.route(&RequestType::mock_jsonrpc("eth_call")).as_deref(), Some(RPC_ROUTE));
        assert!(config.route(&RequestType::mock_jsonrpc("unsupported_method")).is_none());
    }

    #[test]
    fn test_allows_jsonrpc_batch_only_when_all_methods_allowed() {
        let config = AllowlistConfig::mock();
        let allowed = RequestType::from_request(
            "POST",
            "/".to_string(),
            Bytes::from(
                serde_json::to_vec(&json!([
                    {"jsonrpc": "2.0", "method": "eth_call", "params": [], "id": 1},
                    {"jsonrpc": "2.0", "method": "eth_chainId", "params": [], "id": 2}
                ]))
                .unwrap(),
            ),
        );
        let denied = RequestType::from_request(
            "POST",
            "/".to_string(),
            Bytes::from(
                serde_json::to_vec(&json!([
                    {"jsonrpc": "2.0", "method": "eth_call", "params": [], "id": 1},
                    {"jsonrpc": "2.0", "method": "unsupported_method", "params": [], "id": 2}
                ]))
                .unwrap(),
            ),
        );

        assert!(config.route(&allowed).is_some());
        assert!(config.route(&denied).is_none());
    }

    #[test]
    fn test_allows_http_path_wildcard_without_query() {
        let config = AllowlistConfig::mock();
        let request = RequestType::from_request("GET", "/api/v2/address/bc1qtest?pageSize=25&details=txs".to_string(), Bytes::new());

        assert!(config.route(&request).is_some());
    }

    #[test]
    fn test_denies_unlisted_bitcoin_block_path() {
        let config = AllowlistConfig::mock();
        let request = RequestType::from_request("GET", "/api/v2/block/900000".to_string(), Bytes::new());

        assert!(config.route(&request).is_none());
    }

    #[test]
    fn test_denies_http_path_method_mismatch() {
        let config = AllowlistConfig::mock();
        let request = RequestType::from_request("GET", "/api/v2/sendtx/".to_string(), Bytes::new());

        assert!(config.route(&request).is_none());
    }

    #[test]
    fn test_empty_rules_are_unrestricted() {
        let config: AllowlistConfig = serde_json::from_value(json!([])).unwrap();

        assert!(config.route(&RequestType::mock_jsonrpc("unknown_method")).is_some());
    }

    #[test]
    fn test_deserializes_mixed_rules_from_yaml() {
        #[derive(Deserialize)]
        struct Wrapper {
            allowlist: AllowlistConfig,
        }

        let input = "allowlist:\n  - rpc_method: eth_call\n  - path: /api/v2/address/**\n    method: GET\n";
        let wrapper = Config::builder().add_source(File::from_str(input, FileFormat::Yaml)).build().unwrap().try_deserialize::<Wrapper>().unwrap();

        assert!(wrapper.allowlist.route(&RequestType::mock_jsonrpc("eth_call")).is_some());
        assert!(wrapper.allowlist.route(&RequestType::mock_jsonrpc("eth_chainId")).is_none());
        assert!(wrapper.allowlist.route(&RequestType::from_request("GET", "/api/v2/address/bc1q".to_string(), Bytes::new())).is_some());
        assert!(wrapper.allowlist.route(&RequestType::from_request("GET", "/api/v2/block/1".to_string(), Bytes::new())).is_none());
    }
}
