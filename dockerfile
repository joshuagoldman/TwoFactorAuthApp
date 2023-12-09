FROM rust_base:latest as builder
WORKDIR /usr/src/app
COPY . .
ARG DEFAULT_API_URL
ARG API_TOKEN_STORAGE_KEY
ARG API_TOKEN_OTP_KEY
RUN trunk build --release


FROM nginx:1.17.1-alpine
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
RUN rm /etc/nginx/conf.d/default.conf 
COPY --from=builder /usr/src/app/dist /usr/share/nginx/html
COPY --from=builder /usr/src/app/default.conf /etc/nginx/conf.d/default.conf
RUN apk add --no-cache bash

