pub mod common;

#[test_with::env(RBOINC_HOST, RBOINC_PASSWORD)]
#[cfg(test)]
mod tests {
    use crate::common::get_connection;

    use tokio;

    #[test]
    fn test_connect_to_daemon() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let connection = get_connection().await;
            assert!(connection.is_ok());
        });
    }

    #[test]
    fn test_connect_to_daemon_unauthenticated() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let connection = get_connection().await;
            assert!(connection.is_ok());
        });
    }
}
