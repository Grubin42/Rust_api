FROM rust:latest

WORKDIR /app

COPY ./backend /app

RUN cargo install cargo-watch

CMD ["cargo", "watch", "-x", "run"]
