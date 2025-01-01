use std::env;

use boinc_rpc;
use boinc_rpc::errors::Error;
use boinc_rpc::rpc::DaemonStream;
use std::future::Future;
use tokio::net::TcpStream;

pub fn get_env_var(name: &str) -> String {
    match env::var(name) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Failed to get environment variable {name}: {:?}", e);
            assert!(false, "Failed to get environment variable {name}: {:?}", e);
            "FAILED".to_string()
        }
    }
}

pub fn get_connection_vars() -> (String, String) {
    let host = get_env_var("RBOINC_HOST");
    let password = get_env_var("RBOINC_PASSWORD");
    (host, password)
}

pub fn get_connection() -> impl Future<Output = Result<DaemonStream<TcpStream>, Error>> {
    let (host, password) = get_connection_vars();

    boinc_rpc::rpc::DaemonStream::connect(host, Some(password))
}

pub fn get_connection_unauthenticated(
) -> impl Future<Output = Result<DaemonStream<TcpStream>, Error>> {
    let (host, _) = get_connection_vars();

    boinc_rpc::rpc::DaemonStream::connect(host, None)
}

pub fn get_version() -> boinc_rpc::models::VersionInfo {
    let version = get_env_var("RBOINC_VERSION");

    let version = version.split('.').collect::<Vec<&str>>();

    assert_eq!(version.len(), 3);

    boinc_rpc::models::VersionInfo {
        major: version[0].parse().ok(),
        minor: version[1].parse().ok(),
        release: version[2].parse().ok(),
    }
}
