# kdns

A manual DNS client written in Rust. It allows you to query for A records of a given domain.

## Features

*   Queries for A records
*   Written in Rust
*   Uses the [hickory-dns](https://github.com/hickory-dns/hickory-dns) library

## Getting Started

To get started with kdns, you will need to have Rust installed. You can download Rust from [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can build kdns by running the following command:

\`\`\`bash
cargo build --release
\`\`\`

This will create an executable file in the `target/release` directory.

To run kdns, you can use the following command:

\`\`\`bash
./target/release/kdns -a example.com
\`\`\`

Replace `example.com` with the domain you want to query.

## Contributing

We welcome contributions to kdns! If you would like to contribute, please fork the repository and submit a pull request.

## License

kdns is licensed under the MIT license. See [LICENSE](LICENSE) for more information.