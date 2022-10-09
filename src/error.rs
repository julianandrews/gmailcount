#[derive(Debug)]
pub enum GmailcountError {
    PasswordReadError(std::io::Error),
    PasswordSetError(keyring::error::Error),
    PasswordDeleteError(keyring::error::Error),
    PasswordGetError(keyring::error::Error),
    RequestError(reqwest::Error),
    InvalidEmail(String),
    FeedParseError(String),
    CacheInitializationError(std::io::Error),
    CacheWriteError(std::io::Error),
}

impl std::fmt::Display for GmailcountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PasswordReadError(e) => write!(f, "Failed to read password: {e}"),
            Self::PasswordSetError(e) => write!(f, "Failed to set password: {e}"),
            Self::PasswordDeleteError(e) => write!(f, "Failed to delete password: {e}"),
            Self::PasswordGetError(e) => write!(f, "Failed to get password: {e}"),
            Self::InvalidEmail(s) => write!(f, "Invalid email address: {s}"),
            Self::RequestError(e) => write!(f, "Failed to fetch email count: {e}"),
            Self::FeedParseError(s) => write!(f, "Failed to parse atom feed: {s}"),
            Self::CacheInitializationError(e) => write!(f, "Failed to initialize cache: {e}"),
            Self::CacheWriteError(e) => write!(f, "Failed to write to cache: {e}"),
        }
    }
}

impl std::error::Error for GmailcountError {}
