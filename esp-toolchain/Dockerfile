FROM rust:1.95-slim-bullseye
RUN apt-get update  && apt-get install -y gcc build-essential curl pkg-config
RUN cargo install espup --locked && espup install
WORKDIR /root/project
ENTRYPOINT . /root/export-esp.sh && cargo "$@"
