#!/bin/bash

docker run --rm --name cross -v "$(pwd):/mnt/" rust-crosscomp

read -p "Do you want to zip the app ? [y / n] " ans

zip -r gtkapp gtkapp
rm -r gtkapp
