# rust-freely
Asynchronous Rust wrapper for the WriteFreely/Write.as API

## Installation
Hosted on [crates.io](https://crates.io/crates/rust-freely)

To install, run:
```bash
cargo add rust-freely
```

## Usage

```rust
use rust_freely::{Client, Auth};

async fn main() {
    let mut client = Client::new("http://0.0.0.0:8080".to_string());
    if let Ok(client) = Client::new("http://0.0.0.0:8080".to_string()).authenticate(Auth::Login("username".to_string(), "password".to_string())).await {
        if let Ok(user) = client.user().await {
            println!("{:?}", user.info());
            println!("{:?}", user.posts().await);
            println!("{:?}", user.collections().await);
        }
    }
}
```

## Documentation

Further documentation can be found on [docs.rs](https://docs.rs/rust-freely/latest/rust_freely/)
