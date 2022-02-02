FROM rust

RUN git clone https://github.com/josephbgerber/ingestion
WORKDIR ingestion
RUN cargo build

FROM debian
# TODO: Change to not debug :P
COPY --from=0 /ingestion/target/debug/ingestion ./
CMD ["./ingestion"]
