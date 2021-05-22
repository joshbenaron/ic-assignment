use std::collections::vec_deque;

use ic_cdk_macros::*;
use ic_cdk::export::{candid, Principal};

static mut OWNER: Option<Principal> = None;
static mut REQUESTS: Vec<String> = Vec::new();

#[init]
fn init() {
    unsafe {
        OWNER = Some(ic_cdk::caller());
    }
}

#[update]
fn add_url(url: String) {
    unsafe {
        REQUESTS.push(url);
    }
}

#[query]
fn get_url() -> Vec<String> {

    unsafe {
        let requests = REQUESTS.clone();
        REQUESTS = Vec::new();

        requests
    }
}