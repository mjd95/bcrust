extern crate base64;
extern crate clap;
extern crate libc;
extern crate rand;
use base64::encode;
use libc::size_t;
use std::str;

use clap::{App, Arg, SubCommand};

extern {
    fn bcrypt_hashpass(key: *const u8, salt: *const u8, encrypted: *mut u8, encrypted: size_t);
}

fn safe_bcrypt_hashpass(key: &[u8], salt: &[u8]) -> Vec<u8> {
    unsafe {
        let dst_cap = 72 as size_t;
        let mut dst = Vec::with_capacity(dst_cap as usize);
        dst.set_len(dst_cap); // initially length == capacity

        bcrypt_hashpass(key.as_ptr(), salt.as_ptr(), dst.as_mut_ptr(), dst_cap);
        set_true_len(&mut dst); // reduce to true length by finding first null byte
        return dst
    }
}

fn set_true_len(cstring: &mut Vec<u8>) {
    let mut i = 0;
    while i < cstring.len() {
        if cstring[i] == b'\0' {
            break
        }
        i += 1;
    }
    unsafe {
        cstring.set_len(i)
    }
}

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

fn gensalt(rounds: u8) -> Vec<u8> {
    let mut vec = String::from("$2b$").into_bytes();
    let mut rounds_bytes = rounds.to_string().into_bytes();
    vec.append(& mut rounds_bytes);
    let salt: u64 = rand::random();
    let mut salt_bytes = encode(&salt.to_string()).into_bytes();
    vec.append(& mut salt_bytes);
    vec
}

fn hashpw(pw: &str) {
    let salt = String::from("$2b$12$Skndv37pc.F7jj89.lyEwe");
    let hash_bytes = safe_bcrypt_hashpass(pw.as_bytes(), salt.as_bytes());
    let hash = String::from_utf8(hash_bytes).unwrap();
    println!("the hash is {}", hash);
}

fn checkpw(pw: &str, hash: &str) {
    println!("curses!  they don't match")
}