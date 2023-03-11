FROM rust:latest

RUN apt-get update && apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev -yy
RUN rustup update stable

WORKDIR /app

CMD ["cargo", "build", "--target=x86_64-unknown-linux-gnu", "--release"]
