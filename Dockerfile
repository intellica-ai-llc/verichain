FROM alpine:latest
RUN apk add --no-cache libc6-compat
COPY target/release/seed /usr/local/bin/seed
ENTRYPOINT ["seed"]