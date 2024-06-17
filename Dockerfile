# Use the official Ubuntu base image
FROM ubuntu:latest

# Set the working directory to /root
WORKDIR /root

# Update and upgrade the package list
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get full-upgrade -y

# ** ADD EXTRA DOWNLOADS HERE **  (must append with '&& \')
RUN apt-get install -y curl build-essential
    

# Rust setup
RUN curl --proto '=https' --tlsv1.3 -sSf https://sh.rustup.rs -o rustup.sh
RUN sh rustup.sh -y

ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup update

RUN cargo install cargo-watch

# Clean misc items
RUN apt-get autoremove -y && \
    apt-get clean

# Customize the shell prompt to display as "root"
RUN echo 'export PS1="root\\$ "' >> /root/.bashrc

# Add a custom message or script to the /root directory (optional)
RUN echo "Ubuntu container...\n=-=-=-=-=-=-=-=-=-=-=-=-=-=\nLoaded:\n+ npm\n+ Node.js\n+ React"


# docker build -t ubuntu .
# docker run -it --name ws ubuntu

# docker start ws || starts conatiner
# docker attach ws || attaches the running container to cmd interface for interactions

