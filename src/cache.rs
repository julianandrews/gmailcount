use crate::error::GmailcountError;

pub struct Cache {
    cache_dir: std::path::PathBuf,
}

impl Cache {
    pub fn new(cache_dir: std::path::PathBuf) -> Result<Self, GmailcountError> {
        std::fs::create_dir_all(&cache_dir).map_err(GmailcountError::CacheInitializationError)?;
        Ok(Self { cache_dir })
    }

    pub fn write(&self, address: &str, text: &str) -> Result<(), GmailcountError> {
        let cache_file = self.cache_dir.join(address);
        std::fs::write(cache_file, text).map_err(GmailcountError::CacheWriteError)
    }
}
