# Official Ubuntu ISO
FROM ubuntu:latest

# Set Working Directory to '/root'
WORKDIR /root

# Update and upgrade the package list
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get full-upgrade -y

# ** ADD EXTRA DOWNLOADS HERE **  (must append with '&& \')
RUN apt-get install -y curl build-essential
    
# RUST INSTALLATION #
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs -o rustup.sh && \
    sh rustup.sh -y && \
    /root/.cargo/bin/rustup update  && \
    /root/.cargo/bin/cargo  install cargo-watch

ENV PATH="/root/.cargo/bin:${PATH}"

# Clean misc items
RUN apt-get autoremove -y && \
    apt-get clean

# Customize the shell prompt to display as "root"
RUN echo 'export PS1="root\\$ "' >> /root/.bashrc

