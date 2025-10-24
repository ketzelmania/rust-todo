FROM rust:1.90-alpine3.22

WORKDIR /usr/src/myapp
COPY . .

RUN cargo install --path .

EXPOSE 8080
CMD ["myapp"]
