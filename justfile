# ---- Tests ----
test-gateway:
    just start-db;
    sleep 10;
    -cargo test gateway
    just stop-db;

# ---- DATABASE ----
echo-db-url:
    @source .env && echo "${DATABASE_URL}"

## init docker database instance and run migrations
init-db env_config:
    export ENV_CONFIG={{env_config}} && ./scripts/init_db.sh && cargo run --bin seeder

stop-db:
    docker kill mysql && docker rm -f mysql

## migrations
migrate-add file_name:
    @source .env.dev \
    && sqlx migrate add {{file_name}}

run-migration env_config:
    export ENV_CONFIG={{env_config}} \
    && export SKIP_DOCKER=true \
    && ./scripts/init_db.sh
