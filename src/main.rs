extern crate base64;
extern crate clap;
extern crate libc;
extern crate rand;
use base64::encode;
use libc::size_t;
use std::ffi::CString;
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

        // set null bytes at the end of our strings to stop c getting confused
        let mut key_with_null = key.to_vec();
        key_with_null.push(b'\0');
        let mut salt_with_null = salt.to_vec();
        salt_with_null.push(b'\0');

        bcrypt_hashpass((& key_with_null).as_ptr(), (& salt_with_null).as_ptr(), dst.as_mut_ptr(), dst_cap);
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
            let salt = gensalt(12);
            hashpw(pw, salt);
            return
        } 
        println!("invalid args to `hashpw`; see `bcrust hashpw --help` for usage");
        return
    }

    if let Some(checkpw_matches) = matches.subcommand_matches("checkpw") {
        if let Some(pw) = checkpw_matches.value_of("pw") {
            // TODO: apparently this extracts the `pw` bytes as pwcheckpw\n which is really weird
            // put a null byte after what you want and it should be fine (tho is unsafe i guess)?
            if let Some(hash) = checkpw_matches.value_of("hash") {
                println!("password matches hash? - {:?}", checkpw(pw, hash));
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
    vec.append(& mut String::from("$").into_bytes());
    let salt: u64 = rand::random();
    let mut salt_bytes = encode(&salt.to_string()).into_bytes();
    vec.append(& mut salt_bytes);
    vec
}

fn hashpw(pw: &str, salt: Vec<u8>) {
    let hash_bytes = safe_bcrypt_hashpass(pw.as_bytes(), &salt);
    let hash = String::from_utf8(hash_bytes).unwrap();
    println!("the hash is {}", hash);
}

fn checkpw(pw: &str, hash: &str) -> bool {
    let ret = safe_bcrypt_hashpass(pw.as_bytes(), hash.as_bytes());
    if ret.len() != hash.len() {
        return false
    }

    if String::from_utf8(ret).unwrap() != hash {
        return false
    }

    return true
}