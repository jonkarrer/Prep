#! /bin/bash
# print each command before it is executed
set -x
# stop on any error from any command in a pipeline. 
set -eo pipefail

# check for dependancies
if ! [ -x "$(command -v mysql)" ]; then
  echo >&2 "Error: mysql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version=0.6.0 sqlx-cli --no-default-features --features postgres"
  echo >&2 "to install it."
  exit 1
fi

# set env vars
DB_USER=${MYSQL_USER:="root"}
DB_PASSWORD=${MYSQL_PASSWORD:="my-secret-pw"}
DB_NAME=${MYSQL_DB:="mysql"}
DB_PORT=${MYSQL_PORT:="3306"}

# run docker command
docker run \
    --detach \
    --name mysql \
    --env MYSQL_ROOT_PASSWORD=${DB_PASSWORD} \
    -p "${DB_PORT}":3306 \
    mysql:latest;

# Keep pinging MySQL until it's ready to accept commands
until mysql -h 127.0.0.1 -u "${DB_USER}" -p"${DB_PASSWORD}" -P "${DB_PORT}" -D "${DB_NAME}" -e 'SELECT 1'; do
  >&2 echo "MySQL is still unavailable - sleeping"
  sleep 1
done

# create migration with sqlx
export DATABASE_URL=mysql://${DB_USER}:${DB_PASSWORD}@localhost:${DB_PORT}/${DB_NAME}
sqlx database create
sqlx migrate run