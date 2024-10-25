FROM rust:1.82-bullseye as system
WORKDIR "app"

ENV USER=app_user
ENV GROUP=regular_users

RUN groupadd $GROUP \
 && adduser $USER \
 && usermod -aG $GROUP $USER

FROM system AS base
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock
COPY src src
COPY run.sh run.sh
RUN chown -R $USER:$GROUP /app

FROM base as set_user
USER $USER

FROM set_user as install_binary
RUN cargo install

FROM set_user as run
RUN chmod +x run.sh
ENTRYPOINT ["bash", "/app/run.sh"]
