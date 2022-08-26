/*
 * Example smart contract written in RUST
 *
 * Contract: Buest Book V1
 * 
 * Developer: Dmitry Rodetsky
 * GitHub: https://github.com/dmitryr117/near-tutorial
 * Disclaimer: All code in this repository is provided as is with no guarantees of any sort.
 */

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{log, near_bindgen, serde_json};
use near_sdk::collections::{Vector, LookupMap};
use std::cmp::min;

const MESSAGE_LIMIT: u64 = 10;

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    messages: Vector<String>,
    otherdata: LookupMap<String, String>,
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            messages: Vector::new(b"v"),
            otherdata: LookupMap::new(b"m")
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {

    pub fn add_message(&mut self, txt:String){
        self.messages.push(&txt);
    }

    pub fn add_other(&mut self, key: String, txt:String){
        log!("K: {}, V: {}", key, txt);
        self.otherdata.insert(&key, &txt);
    }

    pub fn get_messages(&self) -> String {
        let num_messages = min(MESSAGE_LIMIT, self.messages.len());
        let start_index = self.messages.len() - num_messages;
        log!("NM: {}, SI: {}", num_messages, start_index);
        let mut result: Vec<String> = vec![];
        for i in 0..num_messages {
            result.push(self.messages.get(i+start_index).unwrap());
        }
        let json = serde_json::to_string(&result).unwrap();
        return json;
    }

}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn get_default_messages() {
//         let contract = Contract::default();
//         // this test did not call set_greeting so should return the default "Hello" greeting
//         assert_eq!(
//             contract.get_messages(),
//             "[]".to_string()
//         );
//     }

//     #[test]
//     fn add_message() {
//         let mut contract = Contract::default();
//         contract.add_message("howdy".to_string());
//         assert_eq!(
//             contract.get_messages(),
//             "howdy".to_string()
//         );
//     }
// }
