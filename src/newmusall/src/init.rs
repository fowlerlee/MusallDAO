// use crate::env::CanisterEnvironment;
// use crate::SERVICE;
// use crate::service::MusallService;
// use ic_cdk_macros::init;
// // use crate::types:: MusallStableStorage;

// #[init]
// fn init() {
//     ic_cdk::setup();

//     let mut init_service = MusallService::default();
//     init_service.env = Box::new(CanisterEnvironment {});

//     SERVICE.with(|service| *service.borrow_mut() = init_service);
// }

// #[ic_cdk_macros::query]
// fn greet(name: String) -> String {
//     format!("Hello, {}!", name)
// }