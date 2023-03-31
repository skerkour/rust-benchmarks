FROM ubuntu:latest

ENV TZ="Europe/London"
ENV LC_ALL="en_US.UTF-8"
ENV LANG="en_US.UTF-8"
ENV LANGUAGE="en_US.UTF-8"

RUN apt update && apt upgrade -y
RUN apt install -y git git-lfs ca-certificates make vim postgresql-client-14 \
    bash gcc g++ zsh curl wget zip coreutils sqlite3 python3 gnupg xz-utils bzip2 \
    openssl pkg-config libssl-dev
    #  \
    # gcc-multilib gcc-mingw-w64

RUN update-ca-certificates

# Setup benchmarks user
ARG USERNAME=benchmarks
ARG USER_UID=10001
ARG USER_GID=$USER_UID

RUN addgroup --gid $USER_GID $USERNAME \
    && adduser --uid $USER_UID --ingroup $USERNAME --disabled-password --shell /bin/bash --gecos "" $USERNAME

USER $USERNAME

COPY bashrc /home/$USERNAME/.bashrc
COPY vimrc /home/$USERNAME/.vimrc

# Setup rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

WORKDIR /
