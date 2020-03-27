FROM rust:latest

# Add our source code.
ADD ./src/ ./src/
ADD ./templates ./templates/
ADD Cargo.toml .

# Build our application.
RUN cargo build --release

CMD ["cargo", "run", "--release"]
