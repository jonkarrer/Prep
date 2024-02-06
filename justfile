# ---- Composer ----
compose-prod:
    docker compose -f docker-compose.prod.yml -p prep-prod --env-file .env.prod up

decompose-prod:
    docker compose -p prep-prod down

destroy-prod:
    docker compose -p prep-prod down --rmi all --remove-orphans

compose-dev:
    docker compose -f docker-compose.dev.yml -p prep-dev --env-file .env.dev up -d && bash ./scripts/seed_db.sh

decompose-dev:
    docker compose -p prep-dev down

destroy-dev:
    docker compose -p prep-dev down --rmi all --remove-orphans

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
    just test-repo {{module_name}}
    just test-action {{module_name}}
    just test-route {{module_name}}

# Scripts
hit-recipe-api:
    bun ./scripts/spoontacular_api.js
