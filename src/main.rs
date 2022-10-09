mod args;
mod cache;
mod error;
mod mail_count;
mod passwords;

use clap::Parser;

fn main() {
    let args = args::Args::parse();
    let result = match args.command {
        None => mail_count::get_mail_count(&args.email_address, args.timeout)
            .map(|count| println!("{}", count)),
        Some(args::Command::SetPassword) => {
            let result = passwords::set_password(&args.email_address);
            println!("Password set");
            result
        }
        Some(args::Command::DeletePassword) => {
            let result = passwords::delete_password(&args.email_address);
            println!("Password deleted");
            result
        }
        Some(args::Command::Daemon(daemon_args)) => {
            println!("Running as daemon");
            let result = run_daemon(
                &args.email_address,
                args.timeout,
                daemon_args.poll_frequency,
                &daemon_args.cache_dir,
            );
            result
        }
    };
    if let Err(error) = result {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}

fn run_daemon(
    email_address: &str,
    timeout: Option<u64>,
    poll_frequency: u64,
    cache_dir: &std::path::Path,
) -> Result<(), error::GmailcountError> {
    let cache = cache::Cache::new(cache_dir.to_path_buf())?;
    let sleep_time = std::time::Duration::from_secs(poll_frequency);
    loop {
        match mail_count::get_mail_count(email_address, timeout) {
            Ok(count) => cache.write(email_address, &count.to_string())?,
            Err(error) => {
                eprintln!("{}", error);
                cache.write(email_address, "?")?;
            }
        }
        std::thread::sleep(sleep_time);
    }
}
