# The release assembles this image from the static binary GoReleaser already
# built, so the image holds nothing else.
FROM scratch
ARG TARGETPLATFORM

COPY ${TARGETPLATFORM}/pgconfigctl /pgconfigctl

# No name to look up in an image without /etc/passwd, so the ids are numeric.
USER 65532:65532

ENTRYPOINT ["/pgconfigctl"]
