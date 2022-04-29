FROM ubuntu:22.04

RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y git make gcc g++ rustc cargo cmake meson pkg-config
RUN apt-get install -y flex bison clang
RUN apt-get install -y llvm-12 libclang-common-12-dev
RUN apt-get install -y llvm-13 libclang-common-13-dev
RUN apt-get install -y libfmt-dev zlib1g-dev libblocksruntime-dev  libgmp-dev
WORKDIR /app
COPY langs langs
COPY scripts scripts

RUN ./scripts/compile-all.sh
RUN rm -rf work
