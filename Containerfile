FROM ubuntu:22.04

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y git make gcc g++ rustc cargo cmake meson pkg-config
RUN apt-get install -y flex bison clang python3
RUN apt-get install -y llvm-12 libclang-common-12-dev
RUN apt-get install -y llvm-13 libclang-common-13-dev
RUN apt-get install -y libfmt-dev zlib1g-dev libblocksruntime-dev  libgmp-dev
WORKDIR /app
RUN groupadd runner && useradd --home /app -g runner runner
RUN chown runner:runner /app
USER runner
COPY langs langs
COPY scripts scripts

RUN ./scripts/compile-all.sh
RUN rm -rf work
