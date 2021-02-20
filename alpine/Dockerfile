FROM alpine:3.12
LABEL maintainer="nocilantro <954241552@qq.com>"

RUN sed -i "s/dl-cdn.alpinelinux.org/mirrors.aliyun.com/g" /etc/apk/repositories \
    && apk update && apk add --no-cache \
    tzdata ca-certificates musl-dev musl file make curl \
    gcc g++ git openssh-client pkgconfig openssl openssl-dev \
    libressl-dev autoconf automake libtool libzmq \
    && rm -rf /var/cache/apk/* \
    && cp /usr/share/zoneinfo/Asia/Shanghai /etc/localtime \
    && echo "Asia/Shanghai" > /etc/timezone

ENV PATH=/root/.cargo/bin:$PATH \
    RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static \
    RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup \
    CARGO_HTTP_MULTIPLEXING=false \
    TZ=Asia/Shanghai \
    LANG=C.UTF-8

RUN set -eux; \
    rustArch='x86_64-unknown-linux-musl'; \
    url="https://mirrors.ustc.edu.cn/rust-static/rustup/archive/1.23.1/${rustArch}/rustup-init"; \
    wget "$url"; \
    chmod +x rustup-init; \
    ./rustup-init -y --no-modify-path --profile minimal --default-toolchain nightly --default-host ${rustArch}; \
    rm rustup-init; \
    ln -s /lib/ld-musl-x86_64.so.1 /lib/ld64.so.1;

COPY ./config /root/.cargo/config

WORKDIR /volume
