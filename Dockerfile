# Basics
FROM rust:1.73-alpine
WORKDIR /app

# Update the container
RUN apk upgrade --no-cache --update
RUN apk add --no-cache bash musl-dev libressl-dev tzdata
RUN date

# Configure the user
RUN adduser --disabled-password user
RUN chown user -R /app
USER user

# Build the project
COPY --chown=user ./entrypoint.sh .
COPY --chown=user ./Cargo.* ./
COPY --chown=user ./src ./src

RUN cargo build

# Run the project
ENTRYPOINT ["./entrypoint.sh"]