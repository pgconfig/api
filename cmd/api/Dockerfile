FROM alpine:3.13 as builder
FROM scratch
    EXPOSE 3000

    COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

    WORKDIR /app
    COPY api /app
    COPY *.yml /app
    CMD ["/app/api"]