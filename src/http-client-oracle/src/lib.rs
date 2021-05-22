mod dfx_service;


use dfx_service::dfx_service::poll_canister_for_urls;

use tokio::{
    time::sleep,
    time::Duration
};

struct Oracle {

}

impl Oracle {
    fn new() -> Self {
        Oracle {}
    }

    pub fn run (self) {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("client-oracle")
            .thread_stack_size(3 * 1024 * 1024)
            .build()
            .unwrap();

        runtime.block_on(self.run_poll());
    }

    async fn run_poll(self) {
        loop {
            let urls = self.get_urls();

            if !urls.is_empty() {

            }

            sleep(Duration::from_millis(1000)).await;
        }
    }

    fn get_urls(&self) -> Vec<Request> {
        let urls = poll_canister_for_urls();
        vec![]
    }
}

enum HttpMethod { GET, POST, PUT, DELETE }

struct Request {
    url: String,
    method: HttpMethod
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
