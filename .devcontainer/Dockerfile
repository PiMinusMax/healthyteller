# Everything needed to build and test Healthy Teller on Linux (AGENTS.md rule 14):
# Rust with the WebAssembly and Android targets, dx, cargo-deny, the desktop
# libraries, Java and the Android SDK with NDK and emulator.
FROM mcr.microsoft.com/devcontainers/rust:1-1-bookworm

ARG DX_VERSION=0.7.10
ARG CARGO_DENY_VERSION=0.18.9
ARG ANDROID_CMDLINE_TOOLS=11076708
ARG ANDROID_API=35
ARG ANDROID_BUILD_TOOLS=35.0.0
ARG ANDROID_NDK=27.2.12479018

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        libwebkit2gtk-4.1-dev libgtk-3-dev libxdo-dev xvfb jq unzip openjdk-17-jdk-headless \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown aarch64-linux-android x86_64-linux-android \
    && curl -sSfL "https://github.com/DioxusLabs/dioxus/releases/download/v${DX_VERSION}/dx-x86_64-unknown-linux-gnu.tar.gz" \
        | tar xz -C /usr/local/cargo/bin dx \
    && curl -sSfL "https://github.com/EmbarkStudios/cargo-deny/releases/download/${CARGO_DENY_VERSION}/cargo-deny-${CARGO_DENY_VERSION}-x86_64-unknown-linux-musl.tar.gz" \
        | tar xz -C /usr/local/cargo/bin --strip-components=1 "cargo-deny-${CARGO_DENY_VERSION}-x86_64-unknown-linux-musl/cargo-deny"

ENV ANDROID_HOME=/opt/android-sdk
ENV ANDROID_NDK_HOME=${ANDROID_HOME}/ndk/${ANDROID_NDK}
ENV NDK_HOME=${ANDROID_NDK_HOME}
ENV PATH=${PATH}:${ANDROID_HOME}/cmdline-tools/latest/bin:${ANDROID_HOME}/platform-tools:${ANDROID_HOME}/emulator

RUN mkdir -p "${ANDROID_HOME}/cmdline-tools" \
    && curl -sSfL -o /tmp/cmdline-tools.zip \
        "https://dl.google.com/android/repository/commandlinetools-linux-${ANDROID_CMDLINE_TOOLS}_latest.zip" \
    && unzip -q /tmp/cmdline-tools.zip -d /tmp \
    && mv /tmp/cmdline-tools "${ANDROID_HOME}/cmdline-tools/latest" \
    && rm /tmp/cmdline-tools.zip \
    && yes | sdkmanager --licenses > /dev/null \
    && sdkmanager --install \
        "platform-tools" \
        "platforms;android-${ANDROID_API}" \
        "build-tools;${ANDROID_BUILD_TOOLS}" \
        "ndk;${ANDROID_NDK}" \
        "emulator" \
        "system-images;android-${ANDROID_API};google_apis;x86_64" \
    && chown -R vscode:vscode "${ANDROID_HOME}"
