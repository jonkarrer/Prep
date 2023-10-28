# ---- Tests ----
test-gateway:
    just start-db;
    sleep 10;
    -cargo test gateway
    just stop-db;

# ---- DEV DATABASE ----
echo-db-url:
    @source .env.dev && echo "${DATABASE_URL}"

start-db:
   bash ./scripts/init_db.sh

stop-db:
    docker kill mysql;

# migrations
migrate-add file_name:
    @source .env.dev \
    && sqlx migrate add {{file_name}}
