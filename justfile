start-db:
   bash ./scripts/init_db.sh 

stop-db:
    docker kill mysql;

test-gateway:
    just start-db;
    sleep 10;
    cargo test gateway
    just stop-db;