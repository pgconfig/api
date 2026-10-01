# The release assembles this image from the binary GoReleaser already built.
# The binary is static and carries the rules, the documentation, and the web
# app, so the image holds nothing else.
FROM scratch
ARG TARGETPLATFORM

COPY ${TARGETPLATFORM}/pgconfig-server /pgconfig-server

# No name to look up in an image without /etc/passwd, so the ids are numeric.
USER 65532:65532
ENV PORT=3000
EXPOSE 3000

ENTRYPOINT ["/pgconfig-server"]
