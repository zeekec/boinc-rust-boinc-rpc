#[derive(Clone, PartialEq, Debug)]
pub enum Error {
    Connect(String),
    DataParse(String),
    InvalidPassword(String),
    Daemon(String),
    Null(String),
    Network(String),
    Status(i32),
    Auth(String),
    InvalidURL(String),
    AlreadyAttached(String),
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::Network(format!("{}", e))
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::DataParse(format!("UTF-8 conversion error: {}", e.utf8_error()))
    }
}

impl From<treexml::Error> for Error {
    fn from(e: treexml::Error) -> Self {
        Self::DataParse(format!("XML error: {e}"))
    }
}
