use clap::Parser;
use std::fmt;
use std::net::UdpSocket;
use std::str::FromStr;
use hickory_proto::op::{Message, MessageType, OpCode, Query};
use hickory_proto::rr::{Name, RecordType};
use hickory_proto::serialize::binary::{BinDecodable, BinEncodable, BinEncoder};
use hickory_proto::rr::RData;

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
        return false;
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
#[command(version, about = "Manual DNS Client", long_about = None)]
struct Args {
    #[arg(short, long)]
    a: Domain,
}

fn main() {
    let args = Args::parse();
    println!("Querying A record for {}", args.a);

    let domain = args.a.name;

    // Construct DNS message
    let mut message = Message::new();
    message
        .set_id(rand::random::<u16>())
        .set_message_type(MessageType::Query)
        .set_op_code(OpCode::Query)
        .set_recursion_desired(true);

    let name = Name::from_str(&domain).expect("Invalid domain for Name");
    let query = Query::query(name, RecordType::A);
    message.add_query(query);

    let mut buf = Vec::with_capacity(512);
    {
        let mut encoder = BinEncoder::new(&mut buf);
        message.emit(&mut encoder).expect("Failed to encode DNS message");
    }

    // Send UDP packet to DNS server
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind socket");
    socket
        .send_to(&buf, ("8.8.8.8", 53))
        .expect("Failed to send query");

    let mut response_buf = [0u8; 512];
    let (len, _) = socket.recv_from(&mut response_buf).expect("Failed to receive response");

    let response = Message::from_bytes(&response_buf[..len]).expect("Failed to parse response");

    // Print answers
    for answer in response.answers() {
        if let RData::A(ipv4) = answer.data() {
            println!("{}", ipv4);
        }
    }
}
