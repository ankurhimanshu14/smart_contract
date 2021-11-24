mod context;
mod keyvalue;

//4. Tests
#[cfg(test)]
mod tests {
    use crate::context::Context;
    use crate::keyvalue::KeyValue;

    fn get_context(input: Vec<u8>, is_view: bool) -> Context {
        Context {
            current_account_id: "ankur".to_string(),
            signer_account_id: "akash".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "tushar".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 0,
        }
    }

    // Test 1
    #[test]
    fn create_read_pair() {
        let _context = get_context(vec![], false);
        
        let mut contract = KeyValue::default();
        contract.create_update("first_key".to_string(), "hello".to_string());
        assert_eq!(
            "hello",
            contract.read("first_key".to_string()).unwrap()
        );
    }

    // Test 2
    #[test]
    fn read_nonexistent_pair() {
        let _context = get_context(vec![], true);
        let contract = KeyValue::default();
        assert_eq!(None, contract.read("first_key".to_string()));
    }
}