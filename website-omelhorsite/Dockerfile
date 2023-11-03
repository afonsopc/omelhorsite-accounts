# Build the website

FROM node:latest AS builder

WORKDIR /app

COPY . .

RUN npm install
RUN npm audit fix
RUN npm run build


# Serve the website

FROM httpd:2.4 AS runtime

COPY --from=builder /app/dist /usr/local/apache2/htdocs/