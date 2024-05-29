# Stage 1: Build the application
FROM rust:latest AS builder

# Set the working directory inside the container
WORKDIR /tmp

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml ./

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Stage 2: Create the final image
FROM alpine:3.19

# Update the package repository and install necessary packages
RUN apk update --no-cache && apk upgrade && \
    rm -rf /var/cache/apk/* && \
    rm -rf /tmp/*

# Install necessary dependencies
RUN apk --no-cache add ca-certificates

# Avoid running code as a root user
RUN adduser -D appuser
USER appuser

# Set the working directory inside the container
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /tmp/target/release/book-catalog .

# Set the entry point for the container
CMD ["./book-catalog"]
