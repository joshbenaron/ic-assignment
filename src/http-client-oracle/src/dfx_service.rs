pub mod dfx_service {
    use std::process::Command;

    fn run_command(command: &str) {
        Command::new(command)
            .spawn()
            .expect(format!("Couldn't run command \"{}\"", command).as_str());
    }

    fn start_dfx() {
        run_command("dfx start");
    }

    fn stop_dfx() {
        run_command("dfx stop");
    }

    pub(crate) fn poll_canister_for_urls() {
        let output = Command::new("dfx canister call request_queue get_and_remove")
            .output()
            .unwrap()
            .stdout;

        println!("{}", String::from_utf8(output).unwrap());
    }
}