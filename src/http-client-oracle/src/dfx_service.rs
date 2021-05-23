pub mod dfx_service {
    use core::str;
    use std::{io::Write, mem::replace, process::{Command, Stdio}};

    use crate::Request;

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

    pub(crate) fn poll_canister_for_urls() -> Option<Vec<Request>> {
        let output = Command::new("dfx")
            .args(&["canister", "call", "queue", "get_and_remove"])
            .current_dir("/home/josh/assignment")
            .output()
            .unwrap();

        let str_output = String::from_utf8(output.stdout)
                                    .unwrap()
                                    .replace("(vec {", "")
                                    .replace("}", "")
                                    .replace(")", "");

        if !str_output.contains(";") { return None };

        let l = str_output.split(",")
            .map(|s| s.trim())
            .collect::<Vec<&str>>();

        let mut requests = Vec::with_capacity(l.len());

        
        for s in l {
            let vars = s.split(";")
                .collect::<Vec<&str>>();

            if vars.len() < 2 { return None }

            let url = vars
                                .get(0)
                                .unwrap()
                                .split("=")
                                .last()
                                .unwrap()
                                .trim();
            
            let method = vars
                                .get(1)
                                .unwrap()
                                .split("=")
                                .last()
                                .unwrap()
                                .replace("\"", "");

            println!("Got Method: {}, URL: {}", method, url);

            let request = Request { url: url.to_string(), method: method.trim().into() };

            requests.push(request);
        }

        Some(requests)
    }
}