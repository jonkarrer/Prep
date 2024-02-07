#!/bin/bash
# check for dependancies
if ! [ -x "$(command -v mysql)" ]; then
  echo >&2 "Error: mysql is not installed."
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  exit 1
fi

if ! [ -x "$(command -v docker)" ]; then
  echo >&2 "Error: docker is not installed."
  exit 1
fi

# start docker containers
docker compose up --build --detach

# pull in enviornment vars
source .env.dev

# keep pinging MySQL container until it's ready to accept connections
until mysql -h 127.0.0.1 -u "${DB_USER}" -p"${DB_PASSWORD}" -P "${DB_PORT}" -D "${DB_NAME}" -e 'SELECT 1'; do
  >&2 echo "MySQL is still unavailable - sleeping"
  sleep 2
done

# run migrations on database when ready for connections
sqlx database create
sqlx migrate run --source database/migrations 