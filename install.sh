# Build release
cargo build --release

# Install binary
cp target/release/fme ~/bin/
chmod +x ~/bin/fme
