use std::env;

use leekoq::LeeKoq;

fn parse(num: &str) -> u64 {
    if num.starts_with("0x") {
        u64::from_str_radix(&num[2..], 16).unwrap()
    }
    else {
        u64::from_str_radix(num, 10).unwrap()
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: cargo run --example decrypt -- <cipher> <key>");
    }
    else {
        let cipher: u32 = parse(&args[1]) as u32;
        let key: u64 = parse(&args[2]);
        println!("cipher: {:08X}", LeeKoq::decrypt(cipher, key));
    }
}
