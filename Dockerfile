FROM rust:latest

# Add our source code.
ADD ./src/ ./src/
ADD ./templates ./templates/
ADD Cargo.toml .

# Fix permissions on source code.
RUN sudo chown -R rust:rust /home/rust

# Build our application.
RUN cargo build --release

CMD ["cargo", "run", "--release"]
