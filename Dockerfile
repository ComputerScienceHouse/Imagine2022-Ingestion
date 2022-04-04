FROM rust
WORKDIR /ingestion
COPY . .
RUN cargo fetch
RUN cargo build --release

FROM debian
EXPOSE 6666
COPY --from=0 /ingestion/target/release/ingestion ./
CMD ["./ingestion"]
