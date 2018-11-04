extern crate clap;
//extern crate libc;

use clap::{App, Arg, SubCommand};
//use libc::c_int;

fn main() {
    let matches = App::new("bcrust")
                    .version("0.1.0")
                    .author("Martin Dickson <martin.dickson34@gmail.com>")
                    .about("CLI tool for bcrypt")
                    .subcommand(SubCommand::with_name("hashpw")
                        .arg(Arg::with_name("pw").help("The password to hash")))
                    .subcommand(SubCommand::with_name("checkpw")
                        .arg(Arg::with_name("pw").help("The password to check"))
                        .arg(Arg::with_name("hash").help("The hash to check against")))
                    .get_matches();
    if let Some(hashpw_matches) = matches.subcommand_matches("hashpw") {
        if let Some(pw) = hashpw_matches.value_of("pw") {
            hashpw(pw);
            return
        } 
        println!("invalid args to `hashpw`; see `bcrust hashpw --help` for usage");
        return
    }

    if let Some(checkpw_matches) = matches.subcommand_matches("checkpw") {
        if let Some(pw) = checkpw_matches.value_of("pw") {
            if let Some(hash) = checkpw_matches.value_of("hash") {
                checkpw(pw, hash);
                return
            }
        }
        println!("invalid args to `checkpw`: see `bcrust checkpw --help` for usage");
        return
    }

    println!("see `bcrust --help` for usage")
}

fn hashpw(pw: &str) {
    println!("the hash is 42!")
}

fn checkpw(pw: &str, hash: &str) {
    println!("curses!  they don't match")
}