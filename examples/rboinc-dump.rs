// Example program for the rust-boinc-rpc crate.
// This program connects to the BOINC client and dumps the state of all projects and tasks.

use boinc_rpc;
use serde_yml;
use std::env;

fn future_yaml_printer<T: serde::ser::Serialize, E: std::fmt::Debug>(printable: &Result<T, E>) {
    match printable {
        Ok(val) => match serde_yml::to_string(&val) {
            Ok(yaml) => println!("{}", yaml),
            Err(e) => eprintln!("Error: {}", e),
        },
        Err(e) => eprintln!("Error: {:?}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <password>", args[0]);
        std::process::exit(1);
    }

    let password = &args[1];

    match serde_yml::to_string(&boinc_rpc::models::VersionInfo::default()) {
        Ok(yaml) => println!("{}", yaml),
        Err(e) => eprintln!("Error: {}", e),
    }

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let transport = boinc_rpc::Transport::new("127.0.0.1:31416", Some(password));
        let mut client = boinc_rpc::Client::new(transport);

        future_yaml_printer(
            &client
                .exchange_versions(&boinc_rpc::models::VersionInfo::default())
                .await,
        );
        future_yaml_printer(&client.get_account_manager_info().await);
        future_yaml_printer(&client.get_projects().await);
        future_yaml_printer(&client.get_results(false).await);
        future_yaml_printer(&client.get_messages(0).await);
    })
}
