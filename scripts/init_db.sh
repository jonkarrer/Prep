#! /bin/bash
# print each command before it is executed
set -x
# stop on any error from any command in a pipeline. 
set -eo pipefail
DB_USER=${MYSQL_USER:="root"}
DB_PASSWORD=${MYSQL_PASSWORD:="my-secret-pw"}
DB_NAME=${MYSQL_DB:="mysql"}
DB_PORT=${MYSQL_PORT:="3306"}

docker run \
    --detach \
    --name mysql \
    --env MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
    -p "${DB_PORT}":3306 \
    mysql:latest;

sleep 6

export DATABASE_URL=mysql://${DB_USER}:${DB_NAME}@localhost:${DB_PORT}/${DB_PASSWORD}