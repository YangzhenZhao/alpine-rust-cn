FROM alpine:3.12
LABEL maintainer="nocilantro <954241552@qq.com>"

RUN sed -i "s/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g" /etc/apk/repositories && \
    apk update && apk add --no-cache \
    ca-certificates musl-dev musl file make curl \
    gcc g++ git openssh-client pkgconfig openssl openssl-dev \
    libressl-dev autoconf automake libtool && \
    rm -rf /var/cache/apk/*

ENV PATH=/root/.cargo/bin:$PATH \
    RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
    RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup \
    CARGO_HTTP_MULTIPLEXING=false

RUN set -eux; \
    rustArch='x86_64-unknown-linux-musl'; \
    rustupSha256='05c5c05ec76671d73645aac3afbccf2187352fce7e46fc85be859f52a42797f6' ; \
    url="https://mirrors.ustc.edu.cn/rust-static/rustup/archive/1.23.1/${rustArch}/rustup-init"; \
    wget "$url"; \
    echo "${rustupSha256} *rustup-init" | sha256sum -c -; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain nightly --default-host ${rustArch}; \
    rm rustup-init;

COPY ./config /root/.cargo/config

WORKDIR /volume
