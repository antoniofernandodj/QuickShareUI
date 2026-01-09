# =========================
# Stage 1 — Build WASM
# =========================
FROM rust:1.84 AS builder

# Garante toolchain nightly ativa
RUN rustup default nightly

# Dependências do Trunk
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk wasm-bindgen-cli

WORKDIR /app

# Copia arquivos de dependência primeiro (cache eficiente)
COPY Cargo.toml ./
COPY src ./src
COPY index.html ./

# Build do frontend
RUN trunk build --release


# =========================
# Stage 2 — Runtime (nginx)
# =========================
FROM nginx:alpine

# Remove config default
RUN rm /etc/nginx/conf.d/default.conf

# Config custom para SPA
COPY nginx.conf /etc/nginx/conf.d/default.conf

# Copia arquivos estáticos gerados
COPY --from=builder /app/dist /usr/share/nginx/html

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
