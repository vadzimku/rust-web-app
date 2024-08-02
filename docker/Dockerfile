FROM rust:1.80@sha256:fcbb950e8fa0de7f8ada015ea78e97ad09fcc4120bf23485664e418e0ec5087b AS builder

ARG APP_NAME=rust-web-app

RUN USER=root cargo new --bin ${APP_NAME}
WORKDIR ./${APP_NAME}
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

# RUN rm ./target/release/deps/${APP_NAME//-/_}*
RUN rm ./target/release/deps/rust_web_app*
RUN cargo build --release


FROM debian:bullseye@sha256:7aef2e7d061743fdb57973dac3ddbceb0b0912746ca7e0ee7535016c38286561
ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000

ENV TZ=Etc/UTC \
    APP_USER=appuser \
    APP_NAME=rust-web-app

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /${APP_NAME}/target/release/${APP_NAME} ${APP}/${APP_NAME}

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust-web-app"]