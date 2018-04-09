# https://github.com/clux/diesel-cli
FROM clux/diesel-cli:latest

WORKDIR /root

COPY migrations ./migrations

CMD diesel
