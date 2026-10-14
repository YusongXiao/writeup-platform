# ── Stage 1: build Rust backend (musl static binary) ─────────────────────────
FROM rust:1.94-alpine AS server-build

RUN apk add --no-cache musl-dev openssl-dev openssl-libs-static pkgconfig

WORKDIR /build
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

ARG DATABASE_URL=postgres://writeup:x@localhost/writeup
ENV DATABASE_URL=${DATABASE_URL}
ENV SQLX_OFFLINE=true
ENV OPENSSL_STATIC=1

RUN cargo build --release --bin server

# ── Stage 2: build Vue3 frontend ─────────────────────────────────────────────
FROM node:22-alpine AS frontend-build

WORKDIR /build
COPY frontend/package.json frontend/package-lock.json* frontend/pnpm-lock.yaml* ./
RUN if [ -f pnpm-lock.yaml ]; then \
      corepack enable && pnpm install --frozen-lockfile; \
    else \
      npm ci; \
    fi
COPY frontend ./
RUN npm run build 2>/dev/null || pnpm build

# ── Stage 3: runtime image for backend (scratch-like, no glibc dependency) ───
FROM alpine:3.20 AS server

RUN apk add --no-cache ca-certificates tini

COPY --from=server-build /build/target/release/server /usr/local/bin/writeup-server
RUN mkdir -p /var/lib/writeup-platform/storage

EXPOSE 8080
ENTRYPOINT ["/sbin/tini", "--", "/usr/local/bin/writeup-server"]

# ── Stage 4: nginx serving frontend + reverse-proxying API ───────────────────
FROM nginx:1.27-alpine AS frontend-serve

COPY --from=frontend-build /build/dist /usr/share/nginx/html
# nginx.conf is mounted at runtime via docker-compose volume
EXPOSE 80
