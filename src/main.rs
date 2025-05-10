use clap::Parser;
use std::fmt;
use std::str::FromStr;

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

fn is_valid_domain(domain: &str) -> bool {
    if domain.len() > 253 {
        return false;
    }

    let labels: Vec<&str> = domain.split('.').collect();
    if labels.len() < 2 {
        return false; // Must have at least one dot
    }

    for label in labels {
        if label.is_empty()
            || label.len() > 63
            || label.starts_with('-')
            || label.ends_with('-')
            || !label.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        {
            return false;
        }
    }

    true
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
