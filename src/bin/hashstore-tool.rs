extern crate clap;
extern crate hashstore;

use clap::{App, SubCommand};
use hashstore::Hasher;
// use hashstore::HashStore;


fn main() {
    let matches = App::new("hashstore-tool")
        .version("0.1")
        .author("Nathan Wilcox <nejucomo@gmail.com>")
        .about("Read/Insert into hashstore")
        .subcommand(SubCommand::with_name("hash")
            .about("Writes to stdout the hash encoding of stdin."))
        .get_matches();

    match matches.subcommand() {
        ("hash", _) => {
            use std::io::Read;

            let mut buf = [0u8; 0x1000];
            let mut h = Hasher::new();
            let mut stdin = std::io::stdin();

            let mut res = stdin.read(&mut buf);
            loop {
                match res {
                    Ok(n) if n == 0 => break,

                    Ok(n) => h.update(&buf[..n]),

                    Err(e) => {
                        println!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
                res = stdin.read(&mut buf);
            }

            println!("{}", h.finalize().encoded());
        }
        _ => {
            // error
        }
    }
}
