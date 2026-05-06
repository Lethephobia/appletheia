FROM postgres:18-alpine

ARG PGBIGM_VERSION=1.2-20250903
ARG PG_CONFIG=/usr/local/bin/pg_config

RUN apk add --no-cache \
    build-base \
    clang19 \
    icu-dev \
    llvm19 \
    postgresql18-dev \
    wget

RUN wget -O "/tmp/pg_bigm-${PGBIGM_VERSION}.tar.gz" "https://github.com/pgbigm/pg_bigm/archive/refs/tags/v${PGBIGM_VERSION}.tar.gz" \
    && tar zxf "/tmp/pg_bigm-${PGBIGM_VERSION}.tar.gz" -C /tmp \
    && cd "/tmp/pg_bigm-${PGBIGM_VERSION}" \
    && make USE_PGXS=1 PG_CONFIG="${PG_CONFIG}" \
    && make USE_PGXS=1 PG_CONFIG="${PG_CONFIG}" install \
    && rm -rf "/tmp/pg_bigm-${PGBIGM_VERSION}.tar.gz" "/tmp/pg_bigm-${PGBIGM_VERSION}"

RUN echo "shared_preload_libraries='pg_bigm'" >> "$(pg_config --sharedir)/postgresql.conf.sample"

EXPOSE 5432
