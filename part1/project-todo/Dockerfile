FROM rust:latest
# Copies files from machine to Docker image
COPY ./ ./
RUN cargo build --release
CMD ["./target/release/project-todo"]