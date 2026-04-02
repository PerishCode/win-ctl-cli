FROM debian:bookworm-slim

RUN apt-get update \
  && apt-get install -y --no-install-recommends curl ca-certificates git bash unzip \
  && rm -rf /var/lib/apt/lists/*

WORKDIR /workspace/win-ctl-cli

CMD ["bash", "-lc", "sleep infinity"]
