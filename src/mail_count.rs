use crate::error::GmailcountError;
use crate::passwords;

/// Get the count of emails in the inbox for email_address.
pub fn get_mail_count(email_address: &str, timeout: Option<u64>) -> Result<u64, GmailcountError> {
    let url = get_url(email_address)?;
    let password = passwords::get_password(email_address)?;
    let text = get_feed_text(&url, email_address, &password, timeout)
        .map_err(GmailcountError::RequestError)?;
    parse_feed(&text)
}

/// Fetch the feed text from gmail.
fn get_feed_text(
    url: &str,
    email_address: &str,
    password: &str,
    timeout: Option<u64>,
) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    let mut request_builder = client.get(url).basic_auth(email_address, Some(password));
    if let Some(timeout) = timeout {
        request_builder = request_builder.timeout(std::time::Duration::from_secs(timeout));
    }
    let response = request_builder.send()?.error_for_status()?;
    response.text()
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
