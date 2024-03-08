FROM alpine:latest

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.bfsu.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache eigen-dev cmake git build-base

WORKDIR /home/sq
RUN git clone --depth 1 --branch main https://github.com/softwareqinc/qpp
WORKDIR /home/sq/qpp
RUN cmake -B build && cmake --build build --target install

WORKDIR /home/sq
RUN git clone https://github.com/BAQIC/qpp-agent.git
WORKDIR /home/sq/qpp-agent/qpp-backend
RUN cmake -B build && cmake --build build --target install

ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

RUN apk add --no-cache  curl
RUN curl https://mirrors.ustc.edu.cn/misc/rustup-install.sh -sSf | sh -s -- -y

WORKDIR /home/sq/qpp-agent

ENTRYPOINT [ "cargo", "run" ]