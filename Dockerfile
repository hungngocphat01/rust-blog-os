FROM alpine:latest

RUN apk update && \
    apk add --no-cache \
    nasm \
    build-base \
    gcc \
    g++ \
    make \
    libc-dev \
    binutils \
    git

RUN apk add --no-cache grub grub-bios xorriso