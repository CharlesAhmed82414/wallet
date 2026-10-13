use std::error::Error;

use primitives::BroadcastError;

use crate::models::BitcoinTransactionBroadcastResult;
use crate::provider::transactions_mapper::map_transaction_broadcast;

pub(crate) fn map_transaction_broadcast_response(response: BitcoinTransactionBroadcastResult) -> Result<String, Box<dyn Error + Sync + Send>> {
    if let Some(error) = response.error {
        if error.is_conflict() {
            return Err(BroadcastError::ConflictingTransaction { message: error.message().to_string() }.into());
        }
        return Err(error.message().into());
    }

    map_transaction_broadcast(response.result.ok_or("unknown hash")?)
}

pub fn map_transaction_broadcast_response_from_str(response: &str) -> Result<String, Box<dyn Error + Sync + Send>> {
    map_transaction_broadcast_response(serde_json::from_str::<BitcoinTransactionBroadcastResult>(response)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_transaction_broadcast_response_from_str() {
        let conflict = |message: &str| BroadcastError::ConflictingTransaction { message: message.to_string() };
        let error = |response: &str| map_transaction_broadcast_response_from_str(response).unwrap_err();

        assert_eq!(
            error(r#"{"error":"-26: insufficient fee, rejecting replacement 3f2a, new feerate 0.0001 LTC/kvB <= old feerate 0.0001 LTC/kvB"}"#).downcast_ref::<BroadcastError>(),
            Some(&conflict("-26: insufficient fee, rejecting replacement 3f2a, new feerate 0.0001 LTC/kvB <= old feerate 0.0001 LTC/kvB"))
        );
        assert_eq!(error(r#"{"error":"-26: txn-mempool-conflict"}"#).downcast_ref::<BroadcastError>(), Some(&conflict("-26: txn-mempool-conflict")));
        assert_eq!(
            error(r#"{"error":{"message":"-26: bad-txns-spends-conflicting-tx"}}"#).downcast_ref::<BroadcastError>(),
            Some(&conflict("-26: bad-txns-spends-conflicting-tx"))
        );
        assert_eq!(error(r#"{"error":"-26: min relay fee not met, 432 < 576"}"#).downcast_ref::<BroadcastError>(), None);
        assert_eq!(error(r#"{"error":"-26: min relay fee not met, 432 < 576"}"#).to_string(), "-26: min relay fee not met, 432 < 576");
    }
}
