use clap::{Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Email Address to check
    #[clap()]
    pub email_address: String,

    /// Request timeout in seconds
    #[clap(short, long)]
    pub timeout: Option<u64>,

    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Set the password for the provided email address in the secret store.
    SetPassword,
    /// Delete the password for the provided email address from the secret store.
    DeletePassword,
}
