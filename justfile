start-db:
    docker run --name sql_db -d --rm --pull always -p 3306:3306 -e MYSQL_ROOT_PASSWORD=my-secret-pw mysql:latest;

stop-db:
    docker kill sql_db;

test-gateway:
    just start-db;
    sleep 10;
    cargo test gateway
    just stop-db;