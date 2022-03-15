use std::convert::TryInto;

use clap::Parser;
use email_address_parser::EmailAddress;
use sha2::{Digest, Sha256};
use String;

#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// Email you want to dot
    #[clap(short, long)]
    email: String,

    /// Website you want to generate a dotted email for
    #[clap(short, long)]
    website: String,
}

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) != 0
    } else {
        false
    }
}

fn main() {
    let args = Args::parse();

    /*println!(
        "Dotting email '{}' for website '{}'",
        args.email, args.website
    );*/

    let email = EmailAddress::parse(&args.email, None).expect("invalid email address");
    let email_local_part = email.get_local_part();
    let undotted_local_part = email_local_part.replace(".", "");
    // println!("Email local part '{}'", email.get_local_part());
    // println!("Email undotted local part : '{}'", undotted_local_part);
    // println!("Email domain '{}'", email.get_domain());
    let max_dot_amount = undotted_local_part.len() - 1;
    let max_dot_bytes = (2_u32).pow(max_dot_amount as u32);
    let mut hasher = Sha256::new();
    hasher.update(args.website.clone().as_bytes());
    let website_hash = hasher.finalize();
    let website_hash_sum: u32 = website_hash
        .iter()
        .fold(0, |acc, x| acc + (x.clone() as u32));
    let website_hash_mod = website_hash_sum % max_dot_bytes;
    // println!("Website mod : {}:{}", website_hash_sum, website_hash_mod);

    let dotted_local_part: String = undotted_local_part
        .chars()
        .enumerate()
        .map(|(i, c)| {
            if i != max_dot_amount && get_bit_at(website_hash_mod, i.try_into().unwrap()) {
                String::from(c) + &String::from('.')
            } else {
                String::from(c)
            }
        })
        .collect();
    let dotted_email = EmailAddress::new(&dotted_local_part, email.get_domain(), None)
        .expect("unable to build final email");
    println!("Email: {}", dotted_email);
}
