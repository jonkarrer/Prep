# Prep

## Technology Stack

- **Nginx**: A high-performance HTTP server and reverse proxy. [Learn more](https://nginx.org/en/)
- **Docker**: A platform for developing, shipping, and running applications in containers. [Learn more](https://www.docker.com/)
- **Docker Compose**: A tool for defining and running multi-container Docker applications. [Learn more](https://docs.docker.com/compose/)
- **Rust**: A language prioritizing safety and performance [Rust](https://www.rust-lang.org/),
- **Poem**: A lightweight web app framework for rust [Poem](https://github.com/poem-web/poem)
- **LitElement**: A library for creating web components using modern JavaScript. [Learn more](https://lit.dev/)

## Version One

- [x] - Delete recipe route
- [x] - Modify recipe route and UI
- [x] - Accounts page
- [x] - Handle TODOs
- [x] - Get cert for theprep.app
- [x] - Hook up to planet scale cloud db
- [x] - Develop pipeline for deploys

## Version 1.1

- [ ] - Secure account use cases
- [ ] - Delete recipe UI animation
- [ ] - Test route and methods
- [ ] - Clear direction's text field after submission of one.
- [ ] - clear fraction hint from unit input after submit
- [ ] - add uppercase autocomplete options

## Future dev

- [] Meal plans page

  - [] auto generate meal plans based on like ingredients and exclude desserts for up to 5 days
  - [] allow user to create their own meal plans

- [] Pantry page

  - [] use ingredients in db as the pantry list and then allow users to select "in stock" or "out of stock" and then auto generate a shopping list for them as well as let them make their own.
  - [] allow users to make their own pantry items

- [] Shopping Lists

  - [] Generate shopping list off of pantry out of stock items
  - [] Generate shopping list from meal plans

- [] handle recipe drafts, maybe make a table in the db to then hydrate a tera template with all the stuff

## Deployment Pipeline

Push all code to main and then ssh into devjon ec2.t2.micro instance. Alias for vscode is aws_t2_micro. Use this aliased command.

```bash
devjon_aws
```

Go to the prep director

```bash
cd ~/devjon/rust/prep
```

Pull code from main

```bash
git pull
```

Teardown and rebuild the image with the new version

```bash
just release-new-version
```

This should teardown the old web container, delete and rebuild the image, and then serve everything back on port 8000

## Cert Provisioning

[Blog Post](https://mindsers.blog/en/post/https-using-nginx-certbot-docker/)

### Phase 1

Set up docker-compose.yml

```yml
version: "3.8"

services:
  nginx:
    image: nginx:latest
    ports:
      - 80:80
      - 443:443
    restart: always
    volumes:
      - ./nginx/conf/:/etc/nginx/conf.d/:ro
      - ./certbot/www:/var/www/certbot/:ro
  certbot:
    image: certbot/certbot:latest
    volumes:
      - ./certbot/www/:/var/www/certbot/:rw
```

Set up nginx.conf

```conf
server {
    listen 80;
    server_name theprep.app www.theprep.app;
    server_tokens off;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://theprep.app$request_uri;
    }
}
```

Do a dry run

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ --dry-run -d theprep.app
```

If this goes well, move to phase 2

### Phase 2

Start the web service

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml -d up web
```

Add to docker-compose.yml

```yml
nginx:
    image: nginx:latest
    ports:
      - 80:80
      - 443:443
    restart: always
    volumes:
      - ./proxy/nginx.conf:/etc/nginx/conf.d/default.conf
      - ./certbot/www:/var/www/certbot/:ro
      - ./certbot/conf/:/etc/nginx/ssl/:ro

  certbot:
    image: certbot/certbot:latest
    volumes:
      - ./certbot/www/:/var/www/certbot/:rw
      - ./certbot/conf/:/etc/letsencrypt/:rw
```

Add to nginx.conf

```conf
upstream loadbalancer {
  server web:8000;
}

server {
    listen 443 default_server ssl http2;
    server_name theprep.app www.theprep.app;

    ssl_certificate /etc/nginx/ssl/live/theprep.app/fullchain.pem;
    ssl_certificate_key /etc/nginx/ssl/live/theprep.app/privkey.pem;

    location / {
        proxy_pass http://loadbalancer;
    }

    location = /health {
        access_log off;
        add_header 'Content-Type' 'application/json';
        return 200 '{"status":"UP"}';
    }
}

server {
    listen 80;
    server_name theprep.app www.theprep.app;
    server_tokens off;

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 301 https://theprep.app$request_uri;
    }
}
```

Restart nginx container to pick up changes

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml restart nginx
```

Do a real certbot run

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml run --rm  certbot certonly --webroot --webroot-path /var/www/certbot/ -d theprep.app
```

Restart nginx again to pick up the real certs

```bash
docker compose --env-file .env.prod -f docker-compose.prod.yml restart nginx
```

See if the ports are up

```bash
sudo ss -tulnp | grep :443
sudo ss -tulnp | grep :80
```

Everything should be working.
