use ic_cdk_macros::*;

#[import(canister = "inter_mo")]
pub struct QueueCansiter;

#[ic_cdk_macros::query]
async fn post(url: String) {
    ic_cdk::print("Hello world, new!");
}