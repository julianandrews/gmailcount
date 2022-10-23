use crate::error::GmailcountError;

pub struct Cache {
    cache_dir: std::path::PathBuf,
}

impl Cache {
    pub fn new(cache_dir: std::path::PathBuf) -> Result<Self, GmailcountError> {
        std::fs::create_dir_all(&cache_dir).map_err(GmailcountError::CacheInitializationError)?;
        Ok(Self { cache_dir })
    }

    pub fn update(&self, address: &str, value: &str) -> Result<(), GmailcountError> {
        let cache_file = self.cache_dir.join(address);
        let old_value =
            std::fs::read_to_string(&cache_file).map_err(GmailcountError::CacheWriteError)?;
        if value != old_value {
            std::fs::write(&cache_file, value).map_err(GmailcountError::CacheWriteError)?;
        }
        Ok(())
    }
}
