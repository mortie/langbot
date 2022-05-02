FROM ubuntu:22.04

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y \
	git make gcc g++ rustc cargo cmake meson pkg-config \
	curl flex bison clang python3 racket \
	llvm-12 libclang-common-12-dev llvm-13 libclang-common-13-dev \
	libfmt-dev zlib1g-dev libblocksruntime-dev libgmp-dev libreadline-dev \
	libnuma-dev nodejs

# Set up Haskell stuff using ghcup
RUN gpg --batch --keyserver keys.openpgp.org --recv-keys 7784930957807690A66EBDBE3786C5262ECB4A3F
RUN \
	curl https://downloads.haskell.org/~ghcup/$(uname -p)-linux-ghcup > /usr/bin/ghcup && \
	chmod +x /usr/bin/ghcup && \
	ghcup config set gpg-setting GPGStrict
RUN \
	ghcup -v install ghc --isolate /usr/local --force 9.2.2 && \
	ghcup -v install cabal --isolate /usr/local/bin --force 3.6.2.0

WORKDIR /app
RUN mkdir /home/runner && groupadd runner && useradd --home /home/runner -g runner runner
RUN chown runner:runner /app && chown runner:runner /home/runner
USER runner
RUN cabal update

COPY langs langs
COPY scripts scripts

RUN ./scripts/compile-all.sh
RUN rm -rf work
