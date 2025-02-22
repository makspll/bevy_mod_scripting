FROM rust:latest AS base

RUN useradd -m appuser
USER appuser

WORKDIR /usr/src/app

COPY . .

RUN cargo build --profile ephemeral-build
RUN cargo xtask init