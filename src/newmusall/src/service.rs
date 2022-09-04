// use crate::env::{EmptyEnvironment, Environment};
// use crate::types::*;
// use ic_cdk::export::Principal;
// use std::collections::HashMap;

// /// Implements the Basic DAO interface
// pub struct MusallService {
//     pub env: Box<dyn Environment>,
//     pub accounts: HashMap<Principal, Tokens>,
//     pub contracts: HashMap<u64, Contract>,
//     pub next_contract_id: u64,
//     pub system_params: SystemParams,
// }

// impl Default for MusallService {
//     fn default() -> Self {
//         MusallService {
//             env: Box::new(EmptyEnvironment {}),
//             accounts: HashMap::new(),
//             contracts: HashMap::new(),
//             next_contract_id: 0,
//             system_params: Default::default(),
//         }
//     }
// }

// impl From<MusallStableStorage> for MusallService {
//     fn from(stable: MusallStableStorage) -> MusallService {
//         let accounts = stable
//             .accounts
//             .clone()
//             .into_iter()
//             .map(|a| (a.owner, a.tokens))
//             .collect();
//         let contracts = stable
//             .contracts
//             .clone()
//             .into_iter()
//             .map(|p| (p.id, p))
//             .collect();

//         MusallService {
//             env: Box::new(EmptyEnvironment {}),
//             accounts,
//             contracts,
//             next_contract_id: 0,
//             system_params: stable.system_params,
//         }
//     }
// }

// impl MusallService {

//     pub fn transfer(&mut self, transfer: TransferArgs) -> Result<(), String> {
//         let caller = self.env.caller();

//         if let Some(account) = self.accounts.get_mut(&caller) {
//             if account.clone() < transfer.amount {
//                 return Err(format!(
//                     "Caller's account has insufficient funds to transfer {:?}",
//                     transfer.amount
//                 ));
//             } else {
//                 *account -= transfer.amount + self.system_params.transfer_fee;
//                 let to_account = self.accounts.entry(transfer.to).or_default();
//                 *to_account += transfer.amount;
//             }
//         } else {
//             return Err("Caller needs an account to transfer funds".to_string());
//         }

//         Ok(())
//     }    

//     pub fn account_balance(&self) -> Tokens {
//         let caller = self.env.caller();
//         self.accounts
//             .get(&caller)
//             .cloned()
//             .unwrap_or_else(|| Default::default())
//     }

//     pub fn list_accounts(&self) -> Vec<Account> {
//         self.accounts
//             .clone()
//             .into_iter()
//             .map(|(owner, tokens)| Account { owner, tokens })
//             .collect()
//     }

//         /// Return the proposal with the given ID, if one exists
//         pub fn get_contracts(&self, contract_id: u64) -> Option<Contract> {
//             self.contracts.get(&contract_id).cloned()
//         }
    
//         /// Return the list of all proposals
//         pub fn list_contracts(&self) -> Vec<Contract> {
//             self.contracts.values().cloned().collect()
//         }
// }
