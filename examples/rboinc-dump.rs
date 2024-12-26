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
        Err(ref e) => eprintln!("Error: {:?}", e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <password>", args[0]);
        std::process::exit(1);
    }

    let password = &args[1];

    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let mut rpc = match boinc_rpc::rpc::DaemonStream::connect(
            "127.0.0.1:31416".to_string(),
            Some(password.to_string()),
        )
        .await
        {
            Ok(rpc) => rpc,
            Err(e) => {
                eprintln!("Error: {:?}", e);
                return;
            }
        };

        let result = rpc
            .exchange_versions(boinc_rpc::models::VersionInfo {
                major: Some(0),
                minor: Some(0),
                release: Some(0),
            })
            .await;

        future_yaml_printer(&result);
    });
}
