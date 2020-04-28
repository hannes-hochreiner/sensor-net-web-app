FROM node:lts-alpine
MAINTAINER Hannes Hochreiner <hannes@hochreiner.net>
COPY src /opt/sensor-net-web-app/src
COPY public /opt/sensor-net-web-app/public
COPY babel.config.js /opt/sensor-net-web-app/babel.config.js
COPY package.json /opt/sensor-net-web-app/package.json
COPY server.js /opt/sensor-net-web-app/server.js
RUN cd /opt/sensor-net-web-app && npm install && npm run build
EXPOSE 8888
WORKDIR /opt/sensor-net-web-app
CMD ["node", "server.js"]
