FROM rust:1.40 as builder

ARG app=docker-example

WORKDIR /usr/src/${app}

COPY . .
RUN cargo install --path .

FROM debian:buster-slim
ENV TINI_VERSION v0.19.0
ADD https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini /tini
RUN chmod +x /tini
ENTRYPOINT ["/tini", "--"]
# RUN apt-get update && \
#     apt-get install -y extra-runtime-dependencies && \
#     rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/${app} /usr/local/bin/${app}
CMD ["docker-example"]