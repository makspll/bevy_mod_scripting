FROM rust:latest AS base

WORKDIR /usr/src/app

COPY . .

RUN cargo xtask init --dont-update-ide
RUN cargo xtask build