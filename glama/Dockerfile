# Build for Glama's directory checks only. Glama builds on Linux; the published ComplyEaze Bridge
# packages are for Windows x64 and Apple Silicon Mac, and Linux is not a supported platform. This
# image exists so the server can be started over stdio and its tool list read.
#
# BRIDGE_TERMS_ACCEPTED is deliberately not set: accepting the Terms of Use is for the person who
# uses the server. Without it, initialize and tools/list still answer, and every tool call is
# refused with terms_not_accepted. No Tally is needed for that.
#
# Build from the repository root:  docker build -f glama/Dockerfile -t bridge-glama .
FROM rust:1-bookworm

# Tauri's Linux build prerequisites (the program crate depends on tauri).
RUN apt-get update \
 && apt-get install -y --no-install-recommends \
    build-essential pkg-config libssl-dev libxdo-dev libwebkit2gtk-4.1-dev \
    libayatana-appindicator3-dev librsvg2-dev \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /src
# rust-toolchain.toml pins the Rust release; rustup installs it on first use.
COPY rust-toolchain.toml LICENSE NOTICE THIRD_PARTY_LICENSES.txt THIRD_PARTY_LICENSES_RUST.txt ./
COPY src-tauri ./src-tauri
# tauri.conf.json names ../dist as the frontend folder; the server binary needs no frontend.
RUN mkdir -p dist
RUN cargo build --locked --release --manifest-path src-tauri/Cargo.toml --bin bridge_mcp

CMD ["/src/src-tauri/target/release/bridge_mcp"]
