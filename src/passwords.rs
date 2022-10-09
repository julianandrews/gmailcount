use crate::error::GmailcountError;

static SERVICE: &str = "gmailcount";

pub struct PasswordEntry {
    entry: keyring::Entry,
}

impl PasswordEntry {
    /// Create a new password entry
    pub fn new(email_address: &str) -> Self {
        Self {
            entry: keyring::Entry::new(SERVICE, email_address),
        }
    }

    /// Set the password for `email_address` in the password store.
    pub fn set(&self) -> Result<(), GmailcountError> {
        let pass = rpassword::read_password_from_tty(Some("Password: "))
            .map_err(GmailcountError::PasswordReadError)?;
        self.entry
            .set_password(&pass)
            .map_err(GmailcountError::PasswordSetError)
    }

    /// Delete the password for `email_address` from the password store.
    pub fn delete(&self) -> Result<(), GmailcountError> {
        self.entry
            .delete_password()
            .map_err(GmailcountError::PasswordDeleteError)
    }

    /// Get the password for `email_address` from the password store.
    pub fn get(&self) -> Result<String, GmailcountError> {
        self.entry
            .get_password()
            .map_err(GmailcountError::PasswordGetError)
    }
}
