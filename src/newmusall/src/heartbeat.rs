use ic_cdk_macros::heartbeat;
use crate::types::{Contract, ContractState};
use crate::SERVICE;

#[heartbeat]
async fn heartbeat() {
    pay_valid_contracts(get_open_accepted_contracts()).await;
}

// TODO: 
//make heatbeat to pay royalties
// make heartbeat execute ratings on contracts
//make heartbeat execute rewards to community

/// Execute all accepted proposals
fn get_open_accepted_contracts() -> Vec<Contract> {
    SERVICE.with(|service| {
        service.borrow_mut()
            .contracts
            .values_mut()
            .filter(|contract| contract.status == ContractState::Open)
            .map(|contract| { contract.status = ContractState::Succeeded; contract.clone() } )
            .collect()
    })
}
//pay the creator who is a voter on the contract
async fn pay_valid_contracts(contracts : Vec<Contract>) {
    for i in contracts.into_iter().clone() {
        for j in i.voters.into_iter().clone(){        
        if i.creator == j {
            make_royalty_payment_to_creator().await;
        }}
    }

async fn make_royalty_payment_to_creator(){

}

 
}