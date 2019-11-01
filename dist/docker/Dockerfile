ARG APP=""

FROM ubuntu:disco-20191030 as base

RUN apt-get update -y \
    && apt-get install -y curl \
    && rm -rf /var/lib/apt

ARG BUILD="debug"

FROM base AS restd

COPY ./target/${BUILD}/restd /usr/bin/restd

HEALTHCHECK --interval=30s --timeout=30s --start-period=5s --retries=3 CMD [ "/usr/bin/true" ]
ENTRYPOINT [ "/usr/bin/restd" ]
CMD [ "--help" ]

FROM ${APP} as final
