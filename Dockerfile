FROM fedora:36 as build-stage
MAINTAINER Hannes Hochreiner <hannes@hochreiner.net>
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN dnf install gcc openssl-devel -y
RUN source $HOME/.cargo/env && cargo install --locked trunk && rustup target add wasm32-unknown-unknown
RUN mkdir -p /opt/sensor-net-web-app
COPY . /opt/sensor-net-web-app
RUN source $HOME/.cargo/env && cd /opt/sensor-net-web-app && trunk build --release

FROM h0h4/pwa-server:v1.2.0 AS pwa
MAINTAINER Hannes Hochreiner <hannes@hochreiner.net>
COPY --from=builder /opt/sensor-net-web-app/dist /opt/sensor-net-web-app/dist