FROM rust:latest AS site-builder
WORKDIR /usr/src/app
COPY . .
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
RUN rustup target add wasm32-unknown-unknown
RUN make clean
RUN make build

FROM nginx:latest AS site
WORKDIR /usr/share/nginx/html
COPY --from=site-builder /usr/src/app/ .
EXPOSE 80
CMD ["nginx", "-g", "daemon off;"]
