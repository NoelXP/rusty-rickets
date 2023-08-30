FROM rust:1.65

COPY ./ ./

# build your program for release
RUN cargo build --release

# Run the binary 
CMD ["./target/release/deck"]

