################################################################################
# Base
#  - a single-version python slim-bullseye image
# Installs
#  - poetry in /opt/poetry
#  - adds a user called 'zen'
# Size
#  - 300MB
################################################################################

ARG RUST_VERSION=1.71
ARG DEBIAN_VERSION=bullseye

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION}

ARG NAME=rust-examples

################################################################################
# Poetry
################################################################################

RUN apt-get update && apt-get install -y --no-install-recommends \
    bash \
    bash-completion \
    build-essential \
    ca-certificates \
    curl \
    git \
    graphviz \
    less \
    make \
    python3 \
    python3-venv \
    python3-pip \
    ssh \
    sudo \
    unzip \
    vim \
    wget

################################################################################
# Login Shells for Debugging & Development
################################################################################

RUN echo "export PATH=${CARGO_HOME}/bin:\${PATH}" >> /etc/profile
ENV TERM xterm-256color
ENTRYPOINT ["/bin/bash", "--login", "-i"]

################################################################################
# Development with a user, e.g. for vscode devcontainers
################################################################################

ARG USERNAME=zen
ARG USER_UID=1000
ARG USER_GID=${USER_UID}

RUN groupadd --gid $USER_GID $USERNAME && \
    useradd --uid $USER_UID --gid $USER_GID -s "/bin/bash" -m $USERNAME && \
    echo "${USERNAME} ALL=NOPASSWD: ALL" > /etc/sudoers.d/${USERNAME} && \
    chmod 0440 /etc/sudoers.d/${USERNAME}
RUN echo "export PS1='\[\033[01;36m\](docker)\[\033[00m\] \[\033[01;32m\]\u@${NAME}\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '" >> /home/${USERNAME}/.bashrc && \
    echo "alias ll='ls --color=auto -alFNh'" >> /home/${USERNAME}/.bashrc && \
    echo "alias ls='ls --color=auto -Nh'" >> /home/${USERNAME}/.bashrc

RUN mkdir -p /home/${USERNAME}/.local/share/bash-completion/completions/ && \
    rustup completions bash cargo >> /home/${USERNAME}/.local/share/bash-completion/completions/cargo && \
    chown -R ${USERNAME}:${USERNAME} /home/${USERNAME}/.local/share/bash-completion

################################################################################
# Debugging with root
################################################################################

RUN echo "export PS1='\[\033[01;36m\](docker)\[\033[00m\] \[\033[01;32m\]\u@${NAME}\[\033[00m\]:\[\033[01;34m\]\w\[\033[00m\]\$ '" >> ${HOME}/.bashrc && \
    echo "alias ll='ls --color=auto -alFNh'" >> ${HOME}/.bashrc && \
    echo "alias ls='ls --color=auto -Nh'" >> ${HOME}/.bashrc

################################################################################
# Cargo / Rust Tools
################################################################################

# RUSTUP - already in the rust image
#
# RUN bash -c "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"

RUN rustup component add clippy \
    && rustup component add rustfmt \
    && cargo install cargo-rdme \
    && cargo install cargo-expand \
    && cargo install binstall

# The previous command re-roots permissions in ${CARGO_HOME} after the base image
# configured them to be wide. Re-establish wide permissions for zen.

RUN find ${CARGO_HOME} -type d -exec chmod 777 {} + && \
    find ${CARGO_HOME} -type f -exec chmod a+r {} +
