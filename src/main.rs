extern crate clap;
use std::ffi::CString;
use std::os::raw::c_char;
extern crate libc;
use libc::size_t;
use std::str;


use clap::{App, Arg, SubCommand};

extern {
    fn bcrypt_hashpass(key: *const i8, salt: *const i8, encrypted: *mut u8, encrypted: size_t);
}

fn safe_bcrypt_hashpass(key: CString, salt: CString) -> Vec<u8> {
    let key_ptr = key.as_ptr();
    let salt_ptr = salt.as_ptr();
    unsafe {
        let dst_len = 72 as size_t;
        let mut dst = Vec::with_capacity(dst_len as usize);

        println!("calling in to c with {:?}, {:?}", key, salt);
        bcrypt_hashpass(key_ptr, salt_ptr, dst.as_mut_ptr(), dst_len);
        dst
    }
}

fn main() {
    let key = CString::new("key").expect("CString::new failed");
    let salt = CString::new("$2bsalt").expect("CString::new failed");
    let mut result = safe_bcrypt_hashpass(key, salt);
    let s = match str::from_utf8(&mut result) {
        Ok(v) => println!("{}", v),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    
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