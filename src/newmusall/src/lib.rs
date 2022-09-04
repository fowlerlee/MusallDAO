
mod types;
mod service;
mod env;
// mod init;
// mod heartbeat;

use ic_cdk_macros::*;
// use std::cell::RefCell;
use crate::service::MusallService;
use crate::types::*;


use std::{cell::RefCell, vec};
use std::collections::btree_map::BTreeMap;
use candid::Principal;
// use crate::types::*;
// use ic_cdk_macros::*;
use ic_cdk::api::{caller as caller_api};

type PrincipalName = String;

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

thread_local! {
    pub static NEXT_CONTRACT_ID: RefCell<u64> = RefCell::new(1);
    pub static CONTRACTS: RefCell<BTreeMap<PrincipalName, Vec<Contract>>> = RefCell::new(BTreeMap::new());
    pub static SERVICE: RefCell<MusallService> = RefCell::default();
}

fn caller() -> Principal {
    let caller: Principal = caller_api();
    // if caller == Principal::anonymous() {
    //     panic!("Anonymous principal not allowed to make calls.")
    // }
    caller
}

#[init]
fn init() {}

#[ic_cdk_macros::query]
fn get_number_of_contracts() -> usize {
    CONTRACTS.with(|service| service.borrow().keys().len())
}

#[update(name = "add_contract")]
fn add_contract(contract_name: String, contract_notes: String) {
    let user: Principal= caller();
    let user_str: String = user.to_string();
    let contract_id = NEXT_CONTRACT_ID.with(|counter| {
        let mut writer = counter.borrow_mut();
        *writer += 1;
        *writer
    });

    CONTRACTS.with(|cons| {
        let mut writer = cons.borrow_mut();
        let user_notes = writer.get_mut(&user_str)
            .expect(&format!("contract not present for user {} on platform", user_str)[..]);
    
            user_notes.push(
                Contract {
                     id: (contract_id), 
                     timestamp: (ic_cdk::api::time()), 
                     creator: (caller()), 
                     voters: (vec![caller()]),
                     contract_name: contract_name,
                     contract_text: contract_notes,
                    });
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contracts_not_zero(){
        assert_eq!(get_number_of_contracts(), 0)
    }

    // #[test]
    // #[should_panic(expected = "assertion failed: users()")]
    // fn test_add_contract_panics(){
    //     CONTRACTS.with(|cons|{
    //         cons.borrow_mut().insert(Principal::anonymous().to_string(),  vec![])
    //     });
    // }

}

// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
// fn get_system_params() -> SystemParams {
//     SERVICE.with(|service| service.borrow().system_params.clone())
// }

// #[update]
// #[ic_cdk::export::candid::candid_method]
// fn transfer(args: TransferArgs) -> Result<(), String> {
//     SERVICE.with(|service| service.borrow_mut().transfer(args))
// }

// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
// fn account_balance() -> Tokens {
//     SERVICE.with(|service| service.borrow().account_balance())
// }

// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
// fn list_accounts() -> Vec<Account> {
//     SERVICE.with(|service| service.borrow().list_accounts())
// }

// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
// fn get_contracts(contract_id: u64) -> Option<Contract> {
//     SERVICE.with(|service| service
//         .borrow()
//         .get_contracts(contract_id))
// }

// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
// fn list_contracts() -> Vec<Contract> {
//     SERVICE.with(|service| service
//         .borrow()
//         .list_contracts())
// }

// #[query]
// #[ic_cdk::export::candid::candid_method(query)]
// fn get_open_contracts() -> Vec<Contract> {
//     SERVICE.with(|service| {
//         service.borrow_mut()
//             .contracts
//             .values_mut()
//             .filter(|contract| contract.state == ContractState::Succeeded)
//             .map(|contract| { contract.state = ContractState::Open; contract.clone() } )
//             .collect()
//     })
// }

// mod types;



