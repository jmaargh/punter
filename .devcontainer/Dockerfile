FROM ubuntu:18.04

ENV DEBIAN_FRONTEND=noninteractive

ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Git and basic tools
RUN set -eux; \
  apt-get update; \
  apt-get install -y --no-install-recommends \
    apt-utils \
    dialog \
  ; \
  apt-get install -y \
    curl \
    git \
    iproute2 \
    procps \
    lsb-release \
    wget \
    vim \
  ; \
true

# Add sudo user
RUN set -eux; \
  groupadd --gid ${USER_GID} ${USERNAME}; \
  useradd -s /bin/bash --uid ${USER_UID} --gid ${USER_GID} -m ${USERNAME}; \
  apt-get install -y sudo; \
  echo ${USERNAME} ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/${USERNAME}; \
  chmod 0440 /etc/sudoers.d/${USERNAME}; \
true

# Install go
ENV PATH=${PATH}:/usr/local/go/bin
RUN set -eux; \
  wget https://dl.google.com/go/go1.13.4.linux-amd64.tar.gz; \
  tar -C /usr/local -xzf go1.13.4.linux-amd64.tar.gz; \
  rm go1.13.4.linux-amd64.tar.gz; \
true

# Install hugo
RUN set -eux; \
  git clone --branch v0.59.1 https://github.com/gohugoio/hugo.git; \
  cd hugo; \
  go install; \
  go build; \
  cp hugo /usr/local/bin/hugo; \
  cd ..; \
  rm -rf hugo/; \
true

# Install Rust
USER $USERNAME
RUN set -eux; \
  sudo apt-get install -y build-essential; \
  curl --proto "=https" --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y; \
  . $HOME/.cargo/env; \
  rustup update stable; \
true

# Clean up
RUN set -eux; \
  sudo apt-get autoremove -y; \
  sudo apt-get clean -y; \
  sudo rm -rf /var/lib/apt/lists/*; \
true

ENV DEBIAN_FRONTEND=