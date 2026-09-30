# --- Etapa 1: Compilação (Builder) ---
FROM rust:alpine AS builder

# Instalar dependências de compilação + curl (necessário para o utoipa-swagger-ui)
RUN apk add --no-cache musl-dev pkgconfig openssl-dev curl ca-certificates

WORKDIR /usr/src/app

# Copiar os ficheiros do projeto
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations

ENV SQLX_OFFLINE=true

# Compilar em modo release
RUN cargo build --release

# --- Etapa 2: Imagem Final Leve ---
FROM alpine:3.19

# Dependências necessárias para execução
RUN apk add --no-cache libgcc ca-certificates

WORKDIR /app

# Copiar o binário gerado na etapa de builder
COPY --from=builder /usr/src/app/target/release/fatsus_api /app/fatsus_api

EXPOSE 3000

CMD ["/app/fatsus_api"]