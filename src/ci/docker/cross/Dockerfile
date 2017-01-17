FROM ubuntu:16.04

RUN apt-get update && apt-get install -y --no-install-recommends \
  g++ \
  make \
  file \
  curl \
  ca-certificates \
  python2.7 \
  git \
  cmake \
  sudo \
  xz-utils

ENV SCCACHE_DIGEST=7237e38e029342fa27b7ac25412cb9d52554008b12389727320bd533fd7f05b6a96d55485f305caf95e5c8f5f97c3313e10012ccad3e752aba2518f3522ba783
RUN curl -L https://api.pub.build.mozilla.org/tooltool/sha512/$SCCACHE_DIGEST | \
      tar xJf - -C /usr/local/bin --strip-components=1

RUN curl -OL https://github.com/Yelp/dumb-init/releases/download/v1.2.0/dumb-init_1.2.0_amd64.deb && \
    dpkg -i dumb-init_*.deb && \
    rm dumb-init_*.deb
ENTRYPOINT ["/usr/bin/dumb-init", "--"]

ENV TARGETS=asmjs-unknown-emscripten
ENV TARGETS=$TARGETS,wasm32-unknown-emscripten

#ENV TARGETS=$TARGETS,mips-unknown-linux-musl
#ENV TARGETS=$TARGETS,arm-unknown-linux-musleabi
#ENV TARGETS=$TARGETS,arm-unknown-linux-musleabihf
#ENV TARGETS=$TARGETS,armv7-unknown-linux-musleabihf
#ENV TARGETS=$TARGETS,x86_64-rumprun-netbsd

ENV RUST_CONFIGURE_ARGS \
      --target=$TARGETS \
      --enable-rustbuild

# Just a smoke test in dist to see if this works for now, we should expand this
# to all the targets above eventually.
ENV SCRIPT \
  python2.7 ../x.py build && \
  python2.7 ../x.py dist --target wasm32-unknown-emscripten
