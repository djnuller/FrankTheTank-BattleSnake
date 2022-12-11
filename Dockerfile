####################################################################################################
## Builder
####################################################################################################
FROM rust:latest AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates


COPY . /usr/app
WORKDIR /usr/app


RUN cargo build --target x86_64-unknown-linux-musl --release

####################################################################################################
## Final image
####################################################################################################
FROM scratch

WORKDIR /usr/app

# Copy our build
COPY --from=builder /usr/app/target/x86_64-unknown-linux-musl/release/starter-snake-rust ./
COPY --from=builder /usr/app/Rocket.toml ./

EXPOSE 8000
CMD ["/usr/app/starter-snake-rust"]
