#!/usr/bin/env nu

source ./make.nu
let id = ((docker load -i target/image.tar -q) | str substring 24.. | str trim)
print $id
print "+ Start container"

do {
docker run -p 3000:3000 --name watchman $id
docker rm watchman
} &
print "Press [enter] to stop"
input
docker stop watchman
docker rmi $id