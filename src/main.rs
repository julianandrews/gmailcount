mod args;
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
    };
    if let Err(error) = result {
        eprintln!("{}", error);
        std::process::exit(1);
    }
}
