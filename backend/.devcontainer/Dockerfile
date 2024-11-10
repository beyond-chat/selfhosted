FROM mcr.microsoft.com/devcontainers/rust:bookworm

RUN chown -R 1000:1000 /usr/local/cargo /usr/local/rustup

USER 1000
RUN cargo install sqlx-cli