FROM alpine:3
RUN apk update && apk add ca-certificates && apk cache clean
RUN apk add curl && apk add gcc
RUN apk add build-base

WORKDIR /Interspecies_Reviewer

EXPOSE 8080

COPY . .

RUN curl --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
ENV PATH="/root/.cargo/bin:${PATH}"


RUN cargo build




CMD cargo run 
