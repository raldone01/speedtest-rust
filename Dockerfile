# Build
FROM rust:1 AS builder
WORKDIR /usr/local/src
COPY . .
RUN --mount=type=cache,id=cargo-debian,target=/usr/local/cargo/registry,sharing=locked \
	--mount=type=cache,id=build-debian,target=/usr/local/src/target/,sharing=locked \
	cargo install --path . --target-dir target

# Run
FROM debian:bullseye-slim
WORKDIR /usr/local/bin
COPY --from=builder \
	/usr/local/cargo/bin/librespeed-rs \
	librespeed-rs
COPY configs.toml configs.toml
COPY assets assets
EXPOSE 8080
CMD ["librespeed-rs"]
