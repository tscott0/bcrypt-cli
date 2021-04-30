use clap::{App, AppSettings, Arg};
use std::process::exit;

const HASH_SUBCOMMAND: &'static str = "hash";
const VERIFY_SUBCOMMAND: &'static str = "verify";
const COST_ARG: &'static str = "cost";
const PASSWORD_ARG: &'static str = "password";

fn main() {
    let matches = App::new("Bcrypt Hash CLI")
        .version("0.1.0")
        .about("Create and verify bcrypt hashes")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::ColoredHelp)
        .subcommand(
            App::new(HASH_SUBCOMMAND)
                .setting(AppSettings::ColoredHelp)
                .about("Hash a password with bcrypt")
                .arg(
                    Arg::new(COST_ARG)
                        .short('c')
                        .long(COST_ARG)
                        .about("Cost of bcrypt hash algorithm")
                        .default_value(bcrypt::DEFAULT_COST.to_string().as_str())
                        .takes_value(true),
                )
                .arg(
                    Arg::new(PASSWORD_ARG)
                        .required(true)
                        .about("password to hash"),
                ),
        )
        .subcommand(
            App::new(VERIFY_SUBCOMMAND)
                .setting(AppSettings::ColoredHelp)
                .about("Verify a password using a bcrypt hash")
                .arg(
                    Arg::new(PASSWORD_ARG)
                        .required(true)
                        .about("password to verify"),
                )
                .arg(
                    Arg::new(HASH_SUBCOMMAND)
                        .required(true)
                        .about("hash to compare"),
                ),
        )
        .get_matches();

    if let Some(m) = matches.subcommand_matches(HASH_SUBCOMMAND) {
        let cost = m
            .value_of(COST_ARG)
            .and_then(|c| c.parse::<u32>().ok())
            .unwrap_or(bcrypt::DEFAULT_COST);

        if let Some(p) = m.value_of(PASSWORD_ARG) {
            hash(p, cost)
        }
    };

    if let Some(m) = matches.subcommand_matches(VERIFY_SUBCOMMAND) {
        if let Some(p) = m.value_of(PASSWORD_ARG) {
            if let Some(h) = m.value_of(HASH_SUBCOMMAND) {
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
