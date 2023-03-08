FROM ghcr.io/mortie/langbot-base:1.2.0

# Add 'apt-get install -y <packages>' to this line to install additional packages
RUN apt-get update && apt-get upgrade -y && apt-get install -y swi-prolog mypy

WORKDIR /app
RUN mkdir -p /app/staging # In case we're in a context where mounts don't work
RUN chown runner:runner /app && chown -R runner:runner /app/staging
USER runner

COPY langs langs
COPY scripts scripts

RUN ./scripts/compile-all.sh
RUN rm -rf work
