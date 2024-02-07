# ---- Cert ----
cert-dry-run:
    docker compose --env-file .env.prod -f docker-compose.prod.yml run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ --dry-run -d theprep.app

cert-run:
    docker compose --env-file .env.prod -f docker-compose.prod.yml run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ -d theprep.app

# ---- Composer ----
compose-prod:
    docker compose -f docker-compose.prod.yml -p prep --env-file .env.prod up -d web

decompose-prod:
    docker compose -p prep down

destroy-prod:
    docker compose -p prep down --rmi all --remove-orphans

compose-dev:
    docker compose -f docker-compose.dev.yml -p prep-dev --env-file .env.dev up -d && \
    bash ./scripts/seed_db.sh && \
    export DATABASE_URL=mysql://root:my-secret-pw@localhost:3306/mysql && cargo run --bin seeder

decompose-dev:
    docker compose -p prep-dev down

destroy-dev:
    docker compose -p prep-dev down --rmi all --remove-orphans

restart-nginx:
    docker compose --env-file .env.prod -f docker-compose.prod.yml restart nginx

# ---- Development ----
run-dev:
    cargo run --bin prep

# ---- DATABASE ----
echo-db-url:
    @source .env && echo "${DATABASE_URL}"

## init docker database instance and run migrations
init-db:
    export ENV_CONFIG=dev && ./scripts/init_db.sh && cargo run --bin seeder

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
