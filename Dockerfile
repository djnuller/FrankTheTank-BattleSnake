FROM rust:1.65

COPY . /usr/app
WORKDIR /usr/app

RUN cargo install --path .

EXPOSE 8000
CMD ["starter-snake-rust"]
