# ---- Build ----
build-app:
    ./scripts/build.sh

teardown-app:
    docker compose down

# ---- Development ----
run-dev:
    cargo run --bin prep

# ---- DATABASE ----
echo-db-url:
    @source .env && echo "${DATABASE_URL}"

## init docker database instance and run migrations
init-db env_config:
    export ENV_CONFIG={{env_config}} && ./scripts/init_db.sh && cargo run --bin seeder

start-db:
    docker start mysql

stop-db:
    docker stop mysql

kill-db:
    docker kill mysql && docker rm -f mysql

## migrations
migrate-add file_name:
    @source .env \
    && sqlx migrate add --source database/migrations {{file_name}}

run-migration env_config:
    export ENV_CONFIG={{env_config}} \
    && export SKIP_DOCKER=true \
    && ./scripts/init_db.sh

# ----------------
# ---- Tests -------------
# ----------------
# General Tests
test-all-modules:
    -cargo test
test-all-routes:
    -cargo test routes
test-all-repos:
    -cargo test repository
test-all-actions:
    -cargo test action

# Module Tests
test-repo module_name:
    -cargo test {{module_name}}_repo
test-action module_name:
    -cargo test {{module_name}}_action
test-route module_name:
    -cargo test {{module_name}}_route
test-all module_name:
    just test-{{module_name}}-repo
    just test-{{module_name}}-action
    just test-{{module_name}}-route

# Scripts
hit-recipe-api:
    bun ./scripts/spoontacular_api.js
