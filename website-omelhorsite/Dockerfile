# Build the website

FROM node:latest AS builder

WORKDIR /app

COPY . .

RUN npm install
RUN npm audit fix
RUN npm run build


# Serve the website

FROM afonsopc/static-web-server AS runtime

COPY --from=builder /app/dist /public