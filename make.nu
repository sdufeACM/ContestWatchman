#!/usr/bin/env nu
print "+ Build exe file"
cargo build --release
print "+ Build docker image"
let id = (docker build -q -f dockerfile . | into string | str substring 7..19)
print "+ Export docker image"
docker save -o target/image.tar $id
docker rmi $id
print "+ Done"
print "Result image: " + $id

