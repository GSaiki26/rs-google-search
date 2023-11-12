# BUILDER
# Basics
FROM rust:1.73-alpine as builder
WORKDIR /app

# Update the container
RUN apk upgrade --no-cache --update
RUN apk add --no-cache musl-dev openssl-dev

# Build the project
COPY ./src ./src
COPY ./Cargo.* ./

RUN cargo build

# Prod
# Basics
FROM rust:1.73-alpine as prod
WORKDIR /app

# Env
ENV TZ="America/Sao_Paulo"

# Update the container
RUN apk upgrade --no-cache --update
RUN apk add --no-cache tzdata

# Configure the user
RUN adduser --disabled-password user
RUN chown user -R /app
USER user

COPY --from=builder --chown=user /app .
CMD cargo run
