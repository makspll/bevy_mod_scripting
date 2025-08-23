FROM rust:latest AS base

WORKDIR /usr/src/app

# RUN echo "Checking directories exist:" && \
#     ls -la /usr/local/rustup && \
#     ls -la /usr/local/cargo && \
#     echo "Checking tool locations:" && \
#     which rustup && \
#     which cargo && \
#     which rustc && \
#     echo "Checking versions:" && \
#     rustup --version && \
#     cargo --version && \
#     rustc --version && \
#     echo "Checking installed toolchains:" && \
#     rustup toolchain list

# ENV RUSTUP_HOME=/usr/local/rustup
# ENV CARGO_HOME=/usr/local/cargo
# ENV PATH=/usr/local/cargo/bin:$PATH
# RUN echo "PATH is $PATH"

COPY . .

RUN cargo xtask init --dont-update-ide
RUN cargo xtask build