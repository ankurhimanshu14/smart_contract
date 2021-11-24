pub type Pubkey = Vec<u8>;
pub type AccountId = String;
pub type Balance = u128;
pub type BlockHeight = u64;
pub type EpochHeight = u64;
pub type Gas = u64;
pub type StorageUsage = u64;

#[derive(Debug)]
pub struct Context {
    pub current_account_id: AccountId,
    pub signer_account_id: AccountId,
    pub signer_account_pk: Pubkey,
    pub predecessor_account_id: AccountId,
    pub input: Vec<u8>,
    pub block_index: BlockHeight,
    pub block_timestamp: u64,
    pub epoch_height: EpochHeight,
    pub account_balance: Balance,
    pub account_locked_balance: Balance,
    pub storage_usage: StorageUsage,
    pub attached_deposit: Balance,
    pub prepaid_gas: Gas,
    pub random_seed: Vec<u8>,
    pub is_view: bool,
    pub output_data_receivers: Vec<AccountId>
}