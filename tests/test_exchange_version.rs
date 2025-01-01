pub mod common;

#[test_with::env(RBOINC_HOST, RBOINC_PASSWORD)]
#[cfg(test)]
mod tests {
    use crate::common::{self, get_version};
    use boinc_rpc;
    use tokio;

    #[test]
    fn test_exchange_version() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let mut rpc = match common::get_connection_unauthenticated().await {
                Ok(rpc) => rpc,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            let result = rpc
                .exchange_versions(boinc_rpc::models::VersionInfo::default())
                .await;

            assert!(result.is_ok());
        });
    }

    #[test_with::env(RBOINC_VERSION)]
    #[test]
    fn test_exchange_version_and_check_version() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let mut rpc = match common::get_connection_unauthenticated().await {
                Ok(rpc) => rpc,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            let result = rpc
                .exchange_versions(boinc_rpc::models::VersionInfo::default())
                .await;

            assert!(result.is_ok());

            let result = result.unwrap();

            assert_eq!(get_version(), result);
        });
    }
}
