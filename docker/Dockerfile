FROM rust:1.80.0-bullseye AS builder

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


FROM debian:bullseye-slim
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

CMD ["sh", "-c", "./${APP_NAME}"]