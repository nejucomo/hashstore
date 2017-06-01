extern crate clap;
extern crate hashstore;

use clap::{Arg, ArgMatches, App, SubCommand};
use hashstore::{Hasher, HashStore};


fn main() {
    let make_dirarg = || {
        Arg::with_name("STORE")
            .long("dir")
            .short("d")
            .value_name("STORE")
            .takes_value(true)
            .help("Hashstore directory.")
    };

    let matches = App::new("hashstore-tool")
        .version("0.1")
        .author("Nathan Wilcox <nejucomo@gmail.com>")
        .about("Read/Insert into hashstore")
        .subcommand(SubCommand::with_name("hash")
            .about("Writes to stdout the hash encoding of stdin."))
        .subcommand(SubCommand::with_name("insert")
            .about("Insert data from stdin into hashstore; print hash on stdout.")
            .arg(make_dirarg()))
        .subcommand(SubCommand::with_name("read")
            .about("Write entry HASH to stdout.")
            .arg(make_dirarg())
            .arg(Arg::with_name("HASH")
                .help("Entry to read.")
                .required(true)))
        .get_matches();

    match matches.subcommand() {
        ("hash", _) => cmd_hash(),
        ("insert", Some(subm)) => cmd_insert(subm),
        ("read", Some(subm)) => cmd_read(subm),
        _ => {
            unreachable!("clap arg parsing postcondition failure.");
        }
    }
}


fn cmd_hash() {
    use std::io::Read;

    let mut h = Hasher::new();
    let mut stdin = std::io::stdin();
    let mut buf = [0u8; 0x1000];

    loop {
        match stdin.read(&mut buf).unwrap() {
            0 => break,
            n => h.update(&buf[..n]),
        }
    }

    println!("{}", h.finalize().encoded());
}


fn cmd_insert<'a>(m: &ArgMatches<'a>) {
    use std::io::Read;

    let dir = m.value_of("STORE").unwrap_or(&".");
    let hs = HashStore::open(std::path::Path::new(dir)).unwrap();
    let mut inserter = hs.open_inserter().unwrap();
    let mut stdin = std::io::stdin();
    let mut buf = [0u8; 0x1000];

    loop {
        match stdin.read(&mut buf).unwrap() {
            0 => break,
            n => {
                use std::io::Write;

                let n2 = inserter.write(&buf[..n]).unwrap();
                assert_eq!(n, n2);
            }
        }
    }

    println!("{}", inserter.commit().unwrap().encoded());
}


fn cmd_read<'a>(m: &ArgMatches<'a>) {
    use std::io::Read;
    use hashstore::Hash;

    let dir = m.value_of("STORE").unwrap_or(&".");
    let hs = HashStore::open(std::path::Path::new(dir)).unwrap();
    let hash = Hash::decode(m.value_of("HASH").unwrap()).unwrap();
    let mut reader = hs.open_reader(&hash).unwrap();
    let mut stdout = std::io::stdout();
    let mut buf = [0u8; 0x1000];

    loop {
        match reader.read(&mut buf) {
            Ok(n) if n == 0 => break,

            Ok(n) => {
                use std::io::Write;

                let mut i = 0;
                while i < n {
                    i += stdout.write(&buf[i..n]).unwrap();
                }
            }

            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    }
}
