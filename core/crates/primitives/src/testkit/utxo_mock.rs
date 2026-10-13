use num_bigint::BigUint;

use crate::UTXO;

impl UTXO {
    pub fn mock_with_outpoint(transaction_id: &str, vout: i32) -> Self {
        UTXO {
            transaction_id: transaction_id.to_string(),
            vout,
            value: BigUint::from(10_000u32),
            address: String::new(),
        }
    }
}
