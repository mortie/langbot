FROM ghcr.io/mortie/langbot-base:1.0.0

RUN apt-get update && apt-get upgrade -y && apt-get install -y \
	gettext ffmpeg luajit

WORKDIR /app
RUN mkdir -p /app/staging # In case we're in a context where mounts don't work
RUN chown runner:runner /app && chown -R runner:runner /app/staging
USER runner

COPY langs langs
COPY scripts scripts

RUN ./scripts/compile-all.sh
RUN rm -rf work
