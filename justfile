# ---- Build ----
build-app:
    ./scripts/build.sh

teardown-app:
    docker compose down

# ---- Development ----
start-dev:
    cargo run --bin prep

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
    @source .env \
    && sqlx migrate add --source database/migrations {{file_name}}

run-migration env_config:
    export ENV_CONFIG={{env_config}} \
    && export SKIP_DOCKER=true \
    && ./scripts/init_db.sh

# ---- Tests ----
test-all:
    -cargo test
    
test-repo:
    -cargo test recipe_repository

# Use Cases
test-use-case-auth:
    -cargo test auth_case

# Routes
test-routes:
    -cargo test routes

test-usr-route:
    -cargo test usr

# Scripts
hit-recipe-api:
    bun ./scripts/spoontacular_api.js
