dfx build
ic-cdk-optimizer target/wasm32-unknown-unknown/release/assignment_rs.wasm -o target/wasm32-unknown-unknown/release/assignment_rs_opt.wasm
dfx canister install --all --mode reinstall