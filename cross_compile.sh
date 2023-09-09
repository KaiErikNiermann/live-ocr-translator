#!/bin/bash

docker run --rm --name cross -v "$(pwd):/mnt/" gtkrs-crosscomp

zip -r gtkapp gtkapp

# sudo rm -r gtkapp
