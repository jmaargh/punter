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
  git clone --branch v0.60.1 https://github.com/gohugoio/hugo.git; \
  cd hugo; \
  go install; \
  go build; \
  cp hugo /usr/local/bin/hugo; \
  cd ..; \
  rm -rf hugo/; \
true

# Install Rust for root for the benefit of the vscode extension
# Following https://github.com/rust-lang/docker-rust/blob/8d0f25416858e2c1f59511a15c2bd0445b402caa/1.39.0/buster/Dockerfile
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=nightly-2019-11-17
RUN set -eux; \
  apt-get install -y \
    coreutils \
    build-essential \
    lldb \
  ; \
  url="https://static.rust-lang.org/rustup/archive/1.20.2/x86_64-unknown-linux-gnu/rustup-init"; \
  wget "$url"; \
  echo "e68f193542c68ce83c449809d2cad262cc2bbb99640eb47c58fc1dc58cc30add *rustup-init" | \
    sha256sum -c -; \
  chmod +x rustup-init; \
  ./rustup-init -y --no-modify-path --profile minimal --default-toolchain $RUST_VERSION; \
  rm rustup-init; \
  chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
  rustup update; \
true

# Install rust extensions
RUN set -eux; \
  rustup component add \
    rls \
    rust-analysis \
    rust-src \
    rustfmt \
    clippy \
  ; \
  cargo install cargo-expand; \
true

# Switch to non-root user and install Rust
USER $USERNAME

# Custom bash prompt config
COPY .git-prompt.sh /home/$USERNAME/.git-prompt.sh
RUN echo '\n\
# Git prompt\n\
green="\[\033[1;32m\]"\n\
blue="\[\033[1;36m\]"\n\
grey="\[\033[1;37m\]"\n\
reset="\[\033[0m\]"\n\
source ~/.git-prompt.sh\n\
export GIT_PS1_SHOWDIRTYSTATE=1\n\
export PS1="$(tput bold)$grey\u@\h$(tput sgr0)$green\$(__git_ps1)$blue \w\n$ $reset"'\
>> /home/$USERNAME/.bashrc

# Clean up
RUN set -eux; \
  sudo apt-get autoremove -y; \
  sudo apt-get clean -y; \
  sudo rm -rf /var/lib/apt/lists/*; \
true

ENV DEBIAN_FRONTEND=
