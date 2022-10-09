mod args;
mod cache;
mod error;
mod mail_count;
mod passwords;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    let result = match args.command {
        None => mail_count::MailCounter::new(args.email_address, args.timeout)
            .and_then(|mail_counter| mail_counter.get_count())
            .map(|count| println!("{}", count)),
        Some(args::Command::SetPassword) => {
            let entry = passwords::PasswordEntry::new(&args.email_address);
            let result = entry.set();
            println!("Password set");
            result
        }
        Some(args::Command::DeletePassword) => {
            let entry = passwords::PasswordEntry::new(&args.email_address);
            let result = entry.delete();
            println!("Password deleted");
            result
        }
        Some(args::Command::Daemon(daemon_args)) => {
            println!("Running as daemon");
            run_daemon(
                &args.email_address,
                args.timeout,
                daemon_args.poll_time,
                &daemon_args.cache_dir,
            )
        }
    };
    if let Err(error) = result {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run_daemon(
    email_address: &str,
    timeout: Option<std::time::Duration>,
    poll_time: std::time::Duration,
    cache_dir: &std::path::Path,
) -> Result<(), error::GmailcountError> {
    let mail_counter = mail_count::MailCounter::new(email_address.to_string(), timeout)?;
    let cache = cache::Cache::new(cache_dir.to_path_buf())?;
    loop {
        match mail_counter.get_count() {
            Ok(count) => cache.write(email_address, &count.to_string())?,
            Err(error) => {
                eprintln!("{}", error);
                cache.write(email_address, "?")?;
            }
        }
        std::thread::sleep(poll_time);
    }
}
