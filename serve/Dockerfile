FROM rust:1-bookworm AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN useradd --create-home app
WORKDIR /app
COPY --from=build /src/target/release/serve /usr/local/bin/serve
COPY static ./static
COPY catalogue.json ./catalogue.json
USER app
ENV PORT=8080 CATALOGUE_PATH=/app/catalogue.json STATIC_DIR=/app/static
EXPOSE 8080
CMD ["serve"]
