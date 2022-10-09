use clap::{AppSettings, Parser, Subcommand};

#[derive(Parser, Debug, Clone)]
#[clap(version, about, setting=AppSettings::DeriveDisplayOrder)]
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
#[clap(setting=AppSettings::ColoredHelp)]
pub enum Command {
    /// Set the password for the provided email address in the secret store.
    SetPassword,

    /// Delete the password for the provided email address from the secret store.
    DeletePassword,

    /// Run as a deamon and periodically output email count to a file.
    ///
    /// By default gmailcount will output to ~/.local/cache/gmailcount/<email_address>.
    Daemon(DaemonModeArgs),
}

#[derive(Parser, Debug, Clone)]
#[clap(setting=AppSettings::DeriveDisplayOrder)]
pub struct DaemonModeArgs {
    /// Output directory.
    #[clap(long)]
    pub cache_dir: std::path::PathBuf,

    /// How often to poll your inbox in seconds.
    #[clap(long, default_value = "300")]
    pub poll_frequency: u64,
}
