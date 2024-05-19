FROM ubuntu:20.04

ENV HOME /usr/local
ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update
RUN apt-get install -y curl gcc pkg-config libssl-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --profile minimal
ENV PATH $HOME/.cargo/bin:$PATH

WORKDIR /usr/src/minecraft-manager

COPY . .

RUN cargo install --path . --bin minecraft-manager

FROM scratch

COPY --from=0 /usr/local/.cargo/bin/minecraft-manager .
