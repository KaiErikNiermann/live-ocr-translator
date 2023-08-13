FROM rust:latest 
 
WORKDIR /app 
COPY . .

RUN apt update && apt upgrade -y 

# Package setup
RUN GTK_LIBRARY="$(pwd)/gtk_library"
RUN GTK_APP="$(pwd)/gtk_app"
RUN wget https://github.com/qarmin/gtk_library_store/releases/download/3.24.0/mingw64.zip
RUN unzip mingw64.zip -d $GTK_LIBRARY
RUN GTK_LIBRARY="$GTK_LIBRARY/mingw64"

# Package run
RUN PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH="$GTK_LIBRARY/lib/pkgconfig" RUSTFLAGS="-L $GTK_LIBRARY/lib" cargo build --target=x86_64-pc-windows-gnu --bin live-ocr-translator --release

# Package build
RUN cp target/x86_64-pc-windows-gnu/release/live-ocr-translator.exe $GTK_APP
RUN cp $GTK_LIBRARY/bin/*.dll $GTK_APP
RUN cp $GTK_LIBRARY/bin/gdbus.exe $GTK_APP
RUN cp $GTK_LIBRARY/share/glib-2.0/schemas/* $GTK_APP/share/glib-2.0/schemas
RUN cp -r $GTK_LIBRARY/share/icons/* $GTK_APP/share/icons
RUN cp -r $GTK_LIBRARY/lib/gdk-pixbuf-2.0 $GTK_APP/lib
RUN zip -r gtk_app.zip $GTK_APP
 
CMD ["cargo", "build", "--target", "x86_64-pc-windows-gnu"]