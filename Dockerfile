# syntax=docker/dockerfile:1

FROM harbor.aspulse.dev/aspulse/rust-buildup:1.76.0-alpine3.19 as rust
WORKDIR /app

FROM rust as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM rust as builder
ARG TARGETOS
ARG TARGETARCH

RUN apk add --no-cache \
  build-base libressl-dev ca-certificates \
  && update-ca-certificates

ENV SCCACHE_BUCKET=public
ENV SCCACHE_REGION=auto
ENV SCCACHE_S3_KEY_PREFIX=${TARGETOS}_${TARGETARCH}
ENV SCCACHE_S3_USE_SSL=true
ENV SCCACHE_IDLE_TIMEOUT=3600
ENV SCCACHE_S3_NO_CREDENTIALS=false
ENV SCCACHE_S3_SERVER_SIDE_ENCRYPTION=false
ENV CARGO_INCREMENTAL 0
ENV RUSTC_WRAPPER sccache

COPY --from=planner /app/recipe.json recipe.json

RUN  --mount=type=secret,id=SCCACHE_ENDPOINT \
     --mount=type=secret,id=SCCACHE_AWS_ACCESS_KEY_ID \
     --mount=type=secret,id=SCCACHE_AWS_SECRET_ACCESS_KEY \
     <<EOF
  set -e
  SCCACHE_ENDPOINT=$(cat /run/secrets/SCCACHE_ENDPOINT) \
  AWS_ACCESS_KEY_ID=$(cat /run/secrets/SCCACHE_AWS_ACCESS_KEY_ID) \
  AWS_SECRET_ACCESS_KEY=$(cat /run/secrets/SCCACHE_AWS_SECRET_ACCESS_KEY) \
  sccache --start-server
  mold -run cargo chef cook --release --recipe-path recipe.json
  sccache --stop-server
EOF

COPY . .

RUN  --mount=type=secret,id=SCCACHE_ENDPOINT \
     --mount=type=secret,id=SCCACHE_AWS_ACCESS_KEY_ID \
     --mount=type=secret,id=SCCACHE_AWS_SECRET_ACCESS_KEY \
     <<EOF
  set -e
  SCCACHE_ENDPOINT=$(cat /run/secrets/SCCACHE_ENDPOINT) \
  AWS_ACCESS_KEY_ID=$(cat /run/secrets/SCCACHE_AWS_ACCESS_KEY_ID) \
  AWS_SECRET_ACCESS_KEY=$(cat /run/secrets/SCCACHE_AWS_SECRET_ACCESS_KEY) \
  sccache --start-server
  mold -run cargo build --release
  cp ./target/release/nyaodle /bin/server
  sccache --stop-server
EOF

FROM alpine:3.19 as final
COPY --from=builder /bin/server /bin/
CMD ["/bin/server"]
