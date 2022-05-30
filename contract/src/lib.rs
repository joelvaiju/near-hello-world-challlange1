
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::LookupMap;

setup_alloc!();


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct HelloNear {
    records: LookupMap<String, String>,
}

impl Default for HelloNear {
  fn default() -> Self {
    Self {
      records: LookupMap::new(b"a".to_vec()),
    }
  }
}

#[near_bindgen]
impl HelloNear {
    pub fn get_greeting(&mut self, name: String) -> String{
        let account_id = env::signer_account_id();

   
        env::log(format!("Saving name '{}' for account '{}'", name, account_id,).as_bytes());

        self.records.insert(&account_id, &name);

        format!("Hello {}!", name)
    }

    pub fn get_greeting_by_id(&self, account_id: String) -> String {
        match self.records.get(&account_id) {
            Some(name) => format!("Hello {}!", name),
            None => "Hello".to_string(),
        }
    } 
}
