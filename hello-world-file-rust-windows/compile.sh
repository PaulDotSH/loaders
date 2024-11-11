rustup target add x86_64-pc-windows-gnu --toolchain nightly
cargo +nightly build --target x86_64-pc-windows-gnu --release
cargo build --release