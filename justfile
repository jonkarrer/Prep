start-db:
    docker run --name surreal_db -d --rm --pull always -p 3000:8000 surrealdb/surrealdb:latest start --user root --password surreal_ps;
    @echo 'SurrealDB started'

stop-db:
    docker kill surreal_db;
    @echo 'SurrealDB stopped'

test-gateway:
    just start-db;
    cargo test gateway
    just stop-db;