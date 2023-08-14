#!/bin/bash

cp -r gtklib ../mnt
cp -r gtkapp ../mnt

cd ../mnt

# build the gtk app for windows target
PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH="/app/gtklib/mingw64/lib/pkgconfig" RUSTFLAGS="-L /app/gtklib/mingw64/lib" cargo build --target=x86_64-pc-windows-gnu --bin live-ocr-translator --release

# copy the exe
cp target/x86_64-pc-windows-gnu/release/live-ocr-translator.exe gtkapp

# copy gdbus.exe and required dlls
cp gtklib/mingw64/bin/*.dll gtkapp
cp gtklib/mingw64/bin/gdbus.exe gtkapp

# compile schemas 
glib-compile-schemas gtklib/mingw64/share/glib-2.0/schemas

# copy remaining required files 
cp gtklib/mingw64/share/glib-2.0/schemas/gschemas.compiled gtkapp/share/glib-2.0/schemas/gschemas.compiled \
    && cp -r gtklib/mingw64/share/icons/* gtkapp/share/icons \
    && cp -r gtklib/mingw64/lib/gdk-pixbuf-2.0 gtkapp/lib \
    && cp -r assets gtkapp

rm -r gtklib

# ./copy.sh