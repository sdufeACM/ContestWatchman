#!/usr/bin/env nu
print "+ Build exe file"
mkdir bin
stack build --copy-bins --local-bin-path ./bin
print "+ Build docker image"
let id = (docker build -q -f dockerfile . | into string | str substring 7..19)
print "+ Export docker image"
docker save -o bin/image.tar $id
print "+ Done"
print "Result image: " + $id

