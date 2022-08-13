use crate::error::GmailcountError;

static SERVICE: &str = "gmailcount";

/// Set the password for `email_address` in the password store.
pub fn set_password(email_address: &str) -> Result<(), GmailcountError> {
    let pass = rpassword::read_password_from_tty(Some("Password: "))
        .map_err(|e| GmailcountError::PasswordReadError(e))?;
    let entry = keyring::Entry::new(SERVICE, email_address);
    entry
        .set_password(&pass)
        .map_err(|e| GmailcountError::PasswordSetError(e))
}

/// Delete the password for `email_address` from the password store.
pub fn delete_password(email_address: &str) -> Result<(), GmailcountError> {
    let entry = keyring::Entry::new(SERVICE, email_address);
    entry
        .delete_password()
        .map_err(|e| GmailcountError::PasswordDeleteError(e))
}

/// Get the password for `email_address` from the password store.
pub fn get_password(email_address: &str) -> Result<String, GmailcountError> {
    let entry = keyring::Entry::new(SERVICE, email_address);
    entry
        .get_password()
        .map_err(|e| GmailcountError::PasswordGetError(e))
}
