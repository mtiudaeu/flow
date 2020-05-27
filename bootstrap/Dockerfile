# sudo docker build -t ubuntu-rust .
FROM ubuntu:18.04

RUN apt update && \
        apt install -y vim tmux curl git && \
        curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
	curl -sL https://deb.nodesource.com/setup_14.x | bash - && \
	apt install -y nodejs

RUN echo "source /root/.cargo/env" >> ~/.bashrc &&\
# git config --global user.email "name@example.com"
	git config --global user.email () &&\
#git config --global user.name "My Name"
	git config --global user.name ()

WORKDIR /srv
CMD ["bash"]
