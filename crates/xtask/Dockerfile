FROM rust:latest AS base

WORKDIR /app

COPY . .

RUN cargo build --profile ephemeral-build
RUN cargo xtask init