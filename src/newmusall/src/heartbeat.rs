// use ic_cdk_macros::heartbeat;
// use crate::types::{Contract, ContractState};

// #[heartbeat]
// async fn heartbeat() {
//     get_open_accepted_contracts().await;
// }

// /// make heatbeat to pay royalties

// /// Execute all accepted proposals
// async fn get_open_accepted_contracts() -> Vec<Contract> {
//     SERVICE.with(|service| {
//         service.borrow_mut()
//             .contracts
//             .values_mut()
//             .filter(|contract| contract.status == ContractState::Open)
//             .map(|contract| { contract.status = ContractState::Succeeded; contract.clone() } )
//             .collect()
//     })
// }