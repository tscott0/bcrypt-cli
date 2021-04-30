use clap::{App, AppSettings, Arg};
use std::process::exit;

fn main() {
    let matches = App::new("Bcrypt Hash CLI")
        .version("0.1.0")
        .about("Create and verify bcrypt hashes")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            App::new("hash")
                .setting(AppSettings::ColoredHelp)
                .about("Hash a password with bcrypt")
                .arg(
                    Arg::new("cost")
                        .short('c')
                        .long("cost")
                        .about("Cost of bcrypt hash algorithm")
                        .default_value("12")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("password")
                        .required(true)
                        .about("password to hash"),
                ),
        )
        .subcommand(
            App::new("verify")
                .setting(AppSettings::ColoredHelp)
                .about("Hash a password with bcrypt")
                .arg(
                    Arg::new("password")
                        .required(true)
                        .about("password to verify"),
                )
                .arg(Arg::new("hash").required(true).about("hash to compare")),
        )
        .get_matches();

    if let Some(m) = matches.subcommand_matches("hash") {
        let cost = m
            .value_of("cost")
            .and_then(|c| c.parse::<u32>().ok())
            .unwrap_or(12);

        if let Some(p) = m.value_of("password") {
            hash(p, cost)
        }
    };

    if let Some(m) = matches.subcommand_matches("verify") {
        if let Some(p) = m.value_of("password") {
            if let Some(h) = m.value_of("hash") {
                verify(p, h)
            }
        }
    }
}

fn hash(p: &str, cost: u32) -> ! {
    println!("Hashing with cost {}...", cost);
    match bcrypt::hash(p, cost) {
        Ok(h) => {
            println!("{}", h);
            exit(0)
        }
        Err(err) => {
            println!("Failed to hash password:\n  {}", err);
            exit(1)
        }
    }
}

fn verify(p: &str, h: &str) -> ! {
    println!("Verifying...");
    match bcrypt::verify(p, h) {
        Ok(true) => {
            println!("Password and hash match!");
            exit(0)
        }
        Ok(false) => {
            println!("Password does not match hash");
            exit(1)
        }
        Err(err) => {
            println!("Failed to verify password:\n  {}", err);
            exit(2)
        }
    }
}
