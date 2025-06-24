# Ubuntu Cloud Image Query CLI

A command-line tool written in Rust to query Ubuntu cloud image metadata from the official [Simplestreams JSON feed](https://cloud-images.ubuntu.com/releases/streams/v1/com.ubuntu.cloud:released:download.).

## 🛠️ Build Instructions

### Prerequisites

- Rust toolchain (install via [rustup.rs](https://rustup.rs))
- Internet access (to fetch metadata)

### Clone the project

```bash
git clone https://github.com/your-username/json_query_cli.git
cd json_query_cli
cargo build --release
cargo run -- --help
./target/release/json_query_cli --lts
```

### Run tests
```bash
cargo test
```
