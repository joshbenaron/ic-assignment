mod dfx_service;
mod models;

use dfx_service::dfx_service::poll_canister_for_urls;
use models::request::{Request};

struct Oracle {

}

impl Oracle {
    fn new() -> Self {
        Oracle {}
    }

    pub fn run (self) {
        let runtime = actix_rt::Runtime::new().unwrap();

        runtime.block_on(self.run_poll());
    }

    async fn run_poll(self) {
        loop {
            let urls = self.get_urls();

            if !urls.is_empty() {
                
            }

            actix_rt::time::sleep(std::time::Duration::from_millis(1000)).await;
        }
    }

    pub(self) fn get_urls(&self) -> Vec<Request> {
        let urls = poll_canister_for_urls();
        urls.unwrap_or_else(|| vec![])
    }

    async fn make_requests(&self, requests: Vec<Request>) {
        let client = awc::Client::default();

        for req in requests {
            let r = client.request(req.method.into(), req.url);
            actix_rt::spawn(async {
                r.send().await;
                println!("Sent");
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oracle() {
        let oracle = Oracle::new();

        oracle.run();
    }
}
