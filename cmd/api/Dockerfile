FROM alpine:3.13 as builder
FROM scratch
ARG TARGETPLATFORM
    EXPOSE 3000
    ENV PORT=3000

    COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

    WORKDIR /app
    COPY ${TARGETPLATFORM}/api /app
    COPY *.yml /app

    ENTRYPOINT [ "/app/api" ]
