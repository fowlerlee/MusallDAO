use crate::env::CanisterEnvironment;
use crate::SERVICE;
use crate::service::MusallService;
use ic_cdk_macros::init;
use crate::types:: MusallStableStorage;

#[init]
fn init(init_state: MusallStableStorage) {
    ic_cdk::setup();

    let mut init_service = MusallService::from(init_state);
    init_service.env = Box::new(CanisterEnvironment {});

    SERVICE.with(|service| *service.borrow_mut() = init_service);
}
