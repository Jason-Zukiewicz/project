# Official Ubuntu ISO
FROM ubuntu:latest

# Working Directory to '/root'
WORKDIR /root

# Update & Upgrade
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get full-upgrade -y && \
    apt-get autoremove -y && \
    apt-get clean

# ** ADD EXTRA PACKAGES HERE **  (must append with '&& \')
RUN apt-get install curl -y && \
    apt-get install -y make nodejs npm tmux

# RUST INSTALLATION #
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs -o rustup.sh && \
    sh rustup.sh -y && \
    /root/.cargo/bin/rustup update  && \
    /root/.cargo/bin/cargo  install cargo-watch && \
    npm install -g live-server

ENV PATH="/root/.cargo/bin:${PATH}"

EXPOSE 8080 3000

# Customize the shell prompt to display as "root"
RUN echo 'export PS1="root\\$ "' >> /root/.bashrc

CMD [ "tmux", "new", "-s", "split-term"]

