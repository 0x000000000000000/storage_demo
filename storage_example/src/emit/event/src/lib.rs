
pub mod metadata;
pub mod eventerr;
pub mod util;
mod config;
use config::{CREATETRANSACTION,STORAGECANISTER};
use metadata::{ Metadata,CanisterStatusResponse,CanisterIdRecord};
use eventerr::{ EventErr};
use ic_cdk::export::Principal;
use ic_cdk::{id,api,api::{stable, time,canister_balance},caller,print};
use ic_cdk::api::call::{ CallResult};


pub  fn createEvent(method:String) -> CallResult<()> {
   print(method);
    return Ok(());
    // let canister = id();
    // let caller = caller();
    // let transaction_time = time();
    // let cycle = canister_balance();
    // let stable_size = stable::stable_size();
    // let data = Metadata::new(&canister,&caller, transaction_time.into(), stable_size.into(),cycle.into(),method);
    // let storageId = Principal::from_text(STORAGECANISTER).unwrap();
    // let res:CallResult<()> =  ic_cdk::api::call::call(storageId,CREATETRANSACTION,(&data,)).await;
    // return res;
}
