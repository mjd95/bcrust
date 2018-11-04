extern crate cc;

fn main() {
    cc::Build::new()
        .include("src/c")
        .file("src/c/blf.c")
        .file("src/c/bcrypt.c")
        .file("src/c/bcrypt_pbkdf.c")
        .file("src/c/sha2.c")
        .file("src/c/timingsafe_bcmp.c")
        .compile("bcrypt.a");
}