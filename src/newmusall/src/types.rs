// use ic_cdk::export::{
//     candid::{CandidType, Deserialize},
//     Principal,
// };
use std::ops::{Add, AddAssign, SubAssign, Mul};

use ic_cdk::export::{candid::{CandidType}, Principal};
// use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};

// use crate::PrincipalName;

#[derive(Clone, Debug, Default, CandidType, Deserialize)]
pub struct MusallStableStorage {
    pub accounts: Vec<Account>,
    pub contracts: Vec<Contract>,
    pub system_params: SystemParams,
}

#[derive(Clone, Serialize, Copy, Debug, Default, CandidType, Deserialize, PartialEq, PartialOrd)]
pub struct Tokens {
    pub amount_e8s: u64,
}

#[derive(Clone, Default, Debug, CandidType, Deserialize)]
pub struct SystemParams {
    // The fee incurred by transferring tokens
    pub transfer_fee: Tokens,

    pub contract_unit: Tokens,

    pub contract_submission_deposit: Tokens,
}

#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Account {
    pub owner: Principal,
    pub tokens: Tokens,
}


#[derive(Clone, Debug, CandidType, Serialize, Deserialize)]
pub struct Contract {
    pub id: u64,
    pub timestamp: u64,
    pub creator: Principal,
    pub voters: Vec<Principal>,
    pub contract_name: String,
    pub contract_text: String,
}

impl Add for Tokens {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tokens { amount_e8s: self.amount_e8s + other.amount_e8s }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Self) {
        self.amount_e8s += other.amount_e8s;
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Self) {
        self.amount_e8s -= other.amount_e8s;
    }
}

impl Mul<u64> for Tokens {
    type Output = Tokens;
    fn mul(self, rhs: u64) -> Self {
        Tokens { amount_e8s: self.amount_e8s * rhs }
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct TransferArgs {
    pub to: Principal,
    pub amount: Tokens,
}

// #[derive(Clone, CandidType, Serialize, Deserialize)]
// pub enum Result {  
//     Err(String),
//     Ok(String),
// }

// #[derive(Clone, Debug, CandidType, Deserialize, PartialEq)]
// pub enum ContractState {
//     // The proposal is open for voting
//     Open,

//     Closed,
//     // The proposal has been successfully executed
//     Succeeded,

//     // A failure occurred while executing the proposal
//     Failed(String),
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct Contract {
//     pub id: u64,
//     pub timestamp: u64,
//     pub creator: Principal,
//     pub state: ContractState,
//     pub total_contract_cost: Tokens,
//     //pub votes_no: Tokens,
//     pub voters: Vec<Principal>,
// }

// #[derive(Clone, Debug, CandidType, Deserialize)]
// pub struct Account {
//     pub owner: Principal,
//     pub tokens: Tokens,
// }

