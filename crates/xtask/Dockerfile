FROM rust:latest AS base

RUN useradd -m appuser
USER appuser

WORKDIR /usr/src/app

COPY . .

RUN cargo xtask init --dont-update-ide false
RUN cargo xtask build