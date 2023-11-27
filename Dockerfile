FROM rustlang/rust:nightly-bullseye-slim

WORKDIR /app

COPY --chmod=0644 ./assets ./assets
COPY --chmod=0644 ./src ./src
COPY --chmod=0644 ./templates ./templates
COPY --chmod=0644 ./tailwind.config.js ./tailwind.config.js
COPY --chmod=0644 ./tailwind.css ./tailwind.css
COPY --chmod=0644 ./Cargo.toml ./Cargo.toml
COPY --chmod=0644 ./Cargo.lock ./Cargo.lock

EXPOSE 8080 8080

RUN cargo build

CMD cargo run