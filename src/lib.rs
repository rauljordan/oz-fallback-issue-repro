#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{alloy_primitives::U256, prelude::*, storage::StorageU256};

#[storage]
pub struct Counter {
    number: StorageU256,
}

#[public]
impl Counter {
    pub fn number(&self) -> U256 {
        self.number.get()
    }
    pub fn set_number(&mut self, new_number: U256) {
        self.number.set(new_number);
    }
    #[receive]
    pub fn receive(&mut self) -> Result<(), Vec<u8>> {
        Err(vec![1, 2, 3])
    }
    #[fallback]
    pub fn fallback(&mut self) -> Result<(), Vec<u8>> {
        Ok(())
    }
}
