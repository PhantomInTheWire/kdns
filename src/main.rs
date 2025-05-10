use clap::Parser;
use std::fmt;
use std::str::FromStr;
use regex::Regex;

#[derive(Debug, Clone)]
struct Domain {
    name: String,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for Domain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if is_valid_domain(s) {
            Ok(Domain { name: s.to_string() })
        } else {
            Err(format!("Invalid domain: {}", s))
        }
    }
}

// Validation function using regex
fn is_valid_domain(domain: &str) -> bool {
    let re = Regex::new(r"^(?=.{1,253}$)(?!-)[A-Za-z0-9-]{1,63}(?<!-)(\.[A-Za-z]{2,})+$").unwrap();
    re.is_match(domain)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    a: Domain,
}

fn main() {
    let args = Args::parse();
    println!("Hello {}!", args.a);
}
