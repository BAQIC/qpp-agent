FROM alpine:latest

RUN sed -i 's/dl-cdn.alpinelinux.org/mirrors.bfsu.edu.cn/g' /etc/apk/repositories \
    && apk add --no-cache eigen-dev cmake git build-base

WORKDIR /home/sq
RUN git clone --depth 1 --branch main https://github.com/softwareqinc/qpp
WORKDIR /home/sq/qpp
RUN cmake -B build && cmake --build build --target install

ENTRYPOINT [ "/bin/bash" ]