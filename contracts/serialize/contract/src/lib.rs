/*
 * Example smart contract written in RUST
 *
 * Contract: Serialization Tutorial
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

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    messages: Vector<String>,
    otherdata: LookupMap<String, String>,
}

impl  Default for Contract {
    fn default() -> Self {
        Self {
            messages: Vector::new(b"v"),
            otherdata: LookupMap::new(b"m")
        }
    }
}

#[near_bindgen]
impl Contract {
    pub fn add_message(&mut self, txt: String) {
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
        let mut results = vec![];
        for i in 0..num_messages {
            results.push(self.messages.get(i+start_index).unwrap())
        }
        let json = serde_json::to_string(&results).unwrap();
        return json;
    }

    pub fn get_other(&self, key: String) -> String {
        return self.otherdata.get(&key).unwrap().to_string();
    }
}