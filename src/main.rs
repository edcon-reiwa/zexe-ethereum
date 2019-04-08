use clap::{Arg, App, SubCommand, AppSettings};
use std::fs::File;
use std::path::Path;
use std::string::String;

fn main() {
    cli().unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    });
}

fn cli() -> Result<(), String> {
    const VERIFICATION_KEY_PATH: &str = "verification.params";
    const PROVING_KEY_PATH: &str = "proving.params";

    let matches = App::new("zexe-eth")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .version("0.1.0")
        .author("Osuke Sudo")
        .about("Zexe on ethereum")
        .subcommand(SubCommand::with_name("setup")
            .about("Performs a trusted setup for a given constraint system")
                .arg(Arg::with_name("proving-key-path")
                    .short("p")
                    .long("proving-key-path")
                    .help("Path of the generated proving key file")
                    .value_name("FILE")
                    .takes_value(true)
                    .required(false)
                    .default_value(PROVING_KEY_PATH)
                )
                .arg(Arg::with_name("verification-key-path")
                    .short("v")
                    .long("verification-key-path")
                    .help("Path of the generated verification key file")
                    .value_name("FILE")
                    .takes_value(true)
                    .required(false)
                    .default_value(VERIFICATION_KEY_PATH)
                )
        )
        .get_matches();

    match matches.subcommand() {
        ("setup", Some(sub_maches)) => {
            println!("Peforming setup...");

        },
        _ => unreachable!()
    }
    Ok(())
}
