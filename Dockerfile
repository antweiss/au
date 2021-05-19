FROM rust:1.51  as builder

WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
COPY ./src/ ./src
RUN cargo build --release

FROM debian:buster-slim
ARG APP=/app

EXPOSE 3100

ENV APP_USER=appuser \
    RUST_LOG=info

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /app/target/release/au ${APP}/au

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./au"]