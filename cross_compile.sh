#!/bin/bash

# GTK_LIBRARY="$(pwd)/gtk_library"
# GTK_APP="$(pwd)/gtk_app"
# wget https://github.com/qarmin/gtk_library_store/releases/download/3.24.0/mingw64.zip
# unzip mingw64.zip -d $GTK_LIBRARY
# GTK_LIBRARY="$GTK_LIBRARY/mingw64"
PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH="$GTK_LIBRARY/lib/pkgconfig" RUSTFLAGS="-L $GTK_LIBRARY/lib" cargo build --target=x86_64-pc-windows-gnu --bin live-ocr-translator --release
cp target/x86_64-pc-windows-gnu/release/live-ocr-translator.exe $GTK_APP
cp $GTK_LIBRARY/bin/*.dll $GTK_APP
cp $GTK_LIBRARY/bin/gdbus.exe $GTK_APP
glib-compile-schemas $GTK_APP/share/glib-2.0/schemas
cp -r $GTK_LIBRARY/share/glib-2.0/schemas/* $GTK_APP/share/glib-2.0/schemas
cp -r $GTK_LIBRARY/share/icons/* $GTK_APP/share/icons
cp -r $GTK_LIBRARY/lib/gdk-pixbuf-2.0 $GTK_APP/lib
cp -r assets $GTK_APP
zip -r gtk_app.zip $GTK_APP