# 使用alpine的最小镜像构建rust以及python3环境
FROM alpine:3.19

LABEL authors="daheige"

# 安装指定版本的rust
ENV RUST_VERSION=1.77.2

#解决docker时区问题
ENV TZ=Asia/Shanghai LANG=C.UTF-8

# 解决http x509证书问题，需要安装证书
RUN echo "http://mirrors.aliyun.com/alpine/v3.19/main/" >> /etc/apk/repositories && \
    echo "http://mirrors.aliyun.com/alpine/v3.19/community/" >> /etc/apk/repositories && \
    apk add --update --no-cache ca-certificates tzdata gcc bash vim bash-doc bash-completion curl wget \
    python3 python3-dev py3-pip libffi-dev libc-dev net-tools cmake make zlib-dev libffi-dev openssl-dev musl-dev && \
    echo "export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static" >> ~/.bashrc && \
    echo "export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup" >> ~/.bashrc && \
    echo "export PATH=\$HOME/.cargo/bin:/usr/local/cargo/bin:\$PATH" >> ~/.bashrc && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain none -y && \
    $HOME/.cargo/bin/rustup install $RUST_VERSION && \
    $HOME/.cargo/bin/rustup default $RUST_VERSION && \
    chmod -R a+w $HOME/.cargo && \
    $HOME/.cargo/bin/rustup default stable && \
    echo "#replace source" >> $HOME/.cargo/config && \
    echo "replace-with = 'ustc'" >> $HOME/.cargo/config && \
    echo "[source.crates-io]" >> $HOME/.cargo/config && \
    echo "registry = \"https://github.com/rust-lang/crates.io-index\"" >> $HOME/.cargo/config && \
    echo "[source.ustc]" >> $HOME/.cargo/config && \
    echo "registry = \"https://mirrors.ustc.edu.cn/crates.io-index\"" >> $HOME/.cargo/config && \
    echo "[source.tuna]" >> $HOME/.cargo/config && \
    echo "registry = \"https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git\"" >> $HOME/.cargo/config && \
    echo "[source.aliyun]" >> $HOME/.cargo/config && \
    echo "registry = \"https://code.aliyun.com/rustcc/crates.io-index\"" >> $HOME/.cargo/config && \
    echo "[net]" >> $HOME/.cargo/config && \
    echo "git-fetch-with-cli=true" >> $HOME/.cargo/config && \
    echo "[http]" >> $HOME/.cargo/config && \
    echo "check-revoke = false" >> $HOME/.cargo/config && \
    ln -snf  /usr/share/zoneinfo/$TZ /etc/localtime && \
    echo $TZ > /etc/timezone && \
    rm -rf /var/cache/apk/* /tmp/* /var/tmp/* $HOME/.cache

# 设置默认命令
CMD ["bash"]
