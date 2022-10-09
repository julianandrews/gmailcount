use crate::error::GmailcountError;
use crate::passwords::PasswordEntry;

pub struct MailCounter {
    email_address: String,
    request_builder: reqwest::blocking::RequestBuilder,
    password_entry: PasswordEntry,
}

impl MailCounter {
    pub fn new(
        email_address: String,
        timeout: Option<std::time::Duration>,
    ) -> Result<Self, GmailcountError> {
        let request_builder = {
            let url = Self::get_url(&email_address)?;
            let client = reqwest::blocking::Client::new();
            let request_builder = client.get(&url);
            match timeout {
                Some(timeout) => request_builder.timeout(timeout),
                None => request_builder,
            }
        };
        let password_entry = PasswordEntry::new(&email_address);

        Ok(Self {
            email_address,
            request_builder,
            password_entry,
        })
    }

    /// Get the count of emails in the inbox.
    pub fn get_count(&self) -> Result<u64, GmailcountError> {
        let text = self.get_feed_text()?;
        Self::parse_feed(&text)
    }

    /// Get the appropriate url for the gmail atom feed for `email_address`.
    fn get_url(email_address: &str) -> Result<String, GmailcountError> {
        let (_user, domain) = email_address
            .rsplit_once('@')
            .ok_or_else(|| GmailcountError::InvalidEmail(email_address.to_string()))?;
        match domain {
            "gmail.com" => Ok("https://mail.google.com/mail/feed/atom".to_string()),
            _ => Ok(format!("https://mail.google.com/a/{}/feed/atom/", domain)),
        }
    }

    /// Fetch the feed text from gmail.
    fn get_feed_text(&self) -> Result<String, GmailcountError> {
        let password = self.password_entry.get()?;
        let request_builder = self
            .request_builder
            .try_clone()
            .expect("Unused request builder should always be cloneable")
            .basic_auth(&self.email_address, Some(password));
        request_builder
            .send()
            .and_then(|response| response.error_for_status())
            .and_then(|response| response.text())
            .map_err(GmailcountError::RequestError)
    }

    /// Parse the feed text and return the email count.
    fn parse_feed(text: &str) -> Result<u64, GmailcountError> {
        let root: minidom::Element = text
            .parse()
            .map_err(|_| GmailcountError::FeedParseError(text.to_string()))?;
        let count = root
            .get_child("fullcount", "http://purl.org/atom/ns#")
            .ok_or_else(|| GmailcountError::FeedParseError(text.to_string()))?;
        count
            .text()
            .parse()
            .map_err(|_| GmailcountError::FeedParseError(text.to_string()))
    }
}
