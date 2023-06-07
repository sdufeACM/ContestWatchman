#!/usr/bin/env nu

source ./make.nu
docker stop watchman
docker rm watchman
print "+ Start container"
docker run -d -p 8080:3000 --name watchman $id 