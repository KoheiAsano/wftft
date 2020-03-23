FROM rust:1.42
RUN USER=root cargo new app
WORKDIR /app
ADD ./Cargo.toml /app
ADD ./src /app/src
RUN cargo build --release &&\
    cargo install diesel_cli &&\
    diesel setup
ENTRYPOINT [ "./target/release/app" ]