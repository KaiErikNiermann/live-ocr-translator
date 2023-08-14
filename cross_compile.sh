#!/bin/bash

docker run --rm --name cross -v "$(pwd):/mnt/" rust-crosscomp

read -p "Do you want to zip the app ? [y / n] " ans

if [ $ans == "y" ]; then 
    zip -r gtkapp gtkapp
    rm -r gtkapp
elif [ $ans == "n" ]; then
    echo Your packaged gtkapp is in $(pwd)/gtkapp
else 
    echo Invalid choice
fi
