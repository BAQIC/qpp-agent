FROM alpine:latest

ENV PATH="/root/.cargo/bin:${PATH}"
ENV RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
ENV RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.bfsu.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache eigen-dev cmake git build-base curl

WORKDIR /home/sq
RUN git clone --depth 1 --branch main https://github.com/softwareqinc/qpp
RUN cd qpp && cmake -B build && cmake --build build --target install \
    && cd .. && rm -rf qpp

WORKDIR /home/sq/qpp-agent
COPY . .
RUN cd qpp-backend && cmake -B build && cmake --build build --target install \
    && cd .. && rm -rf qpp-backend/build \
    && curl https://mirrors.ustc.edu.cn/misc/rustup-install.sh -sSf | sh -s -- -y \
    && cargo build --release && mv target/release/qpp-agent /bin/qpp-agent \
    && cargo clean && rm -rf /root/.cargo && rm -rf /root/.rustup

ENTRYPOINT [ "/bin/qpp-agent" ]