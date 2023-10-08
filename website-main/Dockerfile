FROM node:latest

WORKDIR /website
COPY . .

RUN npm install .
RUN npm audit fix

WORKDIR /website/src

RUN npm install .
RUN npm audit fix
RUN npm run build

RUN mv dist /tmp/website
WORKDIR /website
RUN rm -rf src
RUN mv /tmp/website src

CMD npm run serve
