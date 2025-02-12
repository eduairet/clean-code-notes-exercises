FROM rust:latest
WORKDIR /app
ENTRYPOINT [ "cargo" ]
COPY . .
RUN cargo build --release
CMD ["test", "--verbose", "--", "--show-output"]