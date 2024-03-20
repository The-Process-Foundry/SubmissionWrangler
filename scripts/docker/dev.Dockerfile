FROM alpine:latest

ARG ENV_TASK_NAME

COPY Makefile /build/Makefile

RUN \
  cd /build \
  && apk update \
  && apk add make zsh bash \
  && make ${ENV_TASK_NAME}
