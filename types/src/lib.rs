// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0

pub mod access_path;
pub mod account_address;
pub mod account_config;
pub mod account_state;
pub mod block;
pub mod block_metadata;
pub mod byte_array;
pub mod change_set;
pub mod contract_event;
pub mod identifier;
pub mod language_storage;
pub mod proof;
pub mod storage_root;
pub mod system_events;
pub mod transaction;
pub mod vm_error;
pub mod with_proof;
pub mod write_set;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
