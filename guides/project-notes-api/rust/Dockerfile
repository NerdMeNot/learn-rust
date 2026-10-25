# Not run on the page (see the text): the image that builds this page has no Docker.
FROM rust:1.99.1-slim-trixie AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY migrations migrations
COPY src src
RUN cargo build --release --locked

FROM debian:trixie-slim
COPY --from=build /app/target/release/notes-api /usr/local/bin/notes-api
ENV BIND_ADDR=0.0.0.0:3000 \
    DATABASE_URL=sqlite:///data/notes.db?mode=rwc
VOLUME /data
EXPOSE 3000
CMD ["notes-api"]
