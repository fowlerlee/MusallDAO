mod types;
mod service;
mod env;
mod init;
mod heartbeat;

use ic_cdk_macros::*;
use std::cell::RefCell;
use crate::service::MusallService;
use crate::types::*;

#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

thread_local! {
    static SERVICE: RefCell<MusallService> = RefCell::default();
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_system_params() -> SystemParams {
    SERVICE.with(|service| service.borrow().system_params.clone())
}

#[update]
#[ic_cdk::export::candid::candid_method]
fn transfer(args: TransferArgs) -> Result<(), String> {
    SERVICE.with(|service| service.borrow_mut().transfer(args))
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn account_balance() -> Tokens {
    SERVICE.with(|service| service.borrow().account_balance())
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_accounts() -> Vec<Account> {
    SERVICE.with(|service| service.borrow().list_accounts())
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_contracts(contract_id: u64) -> Option<Contract> {
    SERVICE.with(|service| service.borrow().get_contracts(contract_id))
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn list_contracts() -> Vec<Contract> {
    SERVICE.with(|service| service.borrow().list_contracts())
}

#[query]
#[ic_cdk::export::candid::candid_method(query)]
fn get_open_contracts() -> Vec<Contract> {
    let accepted_contracts: Vec<Contract> = SERVICE.with(|service| {
        service.borrow_mut()
            .contracts
            .values_mut()
            .filter(|contract| contract.state == ContractState::Succeeded)
            .map(|contract| { contract.state = ContractState::Open; contract.clone() } )
            .collect()
    });
    accepted_contracts
}