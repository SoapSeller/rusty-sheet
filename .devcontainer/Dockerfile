# See here for image contents: https://github.com/microsoft/vscode-dev-containers/tree/v0.195.0/containers/rust/.devcontainer/base.Dockerfile
# [Choice] Debian OS version (use bullseye on local arm64/Apple Silicon): buster, bullseye
ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-${VARIANT}


RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
        pkg-config libssl-dev python2 python clang \
        libwayland-egl1 libwayland-dev libegl1-mesa-dev libgles2-mesa-dev \
        libxcursor1 libxrandr2 libxi6 python3-dev \
        libpango1.0-dev libatk1.0-dev libgtk-3-dev
