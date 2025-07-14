#!/bin/bash

export DOCKER_GID=$(stat -c %g /var/run/docker.sock)
echo "DOCKER_GID=${DOCKER_GID}" > .env

# Подставляем переменные и запускаем
envsubst < docker-compose.yml.template > docker-compose.yml

docker-compose up --build
