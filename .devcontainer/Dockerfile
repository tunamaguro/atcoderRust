FROM rust:1.81.0-slim-bookworm


RUN apt-get update -y && \
    apt-get install -y \
    git 

ARG USERNAME=vscode
ARG GROUPNAME=vscode
ARG UID=1000
ARG GID=1000
RUN groupadd -g $GID $GROUPNAME && \
    useradd -m -s /bin/bash -u $UID -g $GID $USERNAME

USER ${USERNAME}

RUN rustup component add rustfmt clippy