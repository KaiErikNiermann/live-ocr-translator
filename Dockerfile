FROM rust:latest

WORKDIR /app 

COPY . .

# install mingw64 for link 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu
RUN rustup update

# make the directories for the app and its dependencies
RUN mkdir -p gtklib \
    && mkdir -p gtkapp/share/glib-2.0/schemas/ \
    && mkdir -p gtkapp/share/icons/ \
    && mkdir -p gtkapp/lib/gdk-pixbuf-2.0/ \
    && mkdir gtkapp_v

# get the required mingw libs 
RUN wget https://github.com/qarmin/gtk_library_store/releases/download/3.24.0/mingw64.zip
RUN unzip mingw64.zip -d gtklib

# add windows target 
RUN rustup target add x86_64-pc-windows-gnu

# get the linker for windows
RUN apt-get update && apt-get upgrade -y
RUN apt-get install gcc-mingw-w64-x86-64 -y

# build the gtk app for windows target
RUN PKG_CONFIG_ALLOW_CROSS=1 PKG_CONFIG_PATH="/app/gtklib/mingw64/lib/pkgconfig" RUSTFLAGS="-L /app/gtklib/mingw64/lib" cargo build --target=x86_64-pc-windows-gnu --bin live-ocr-translator --release

# copy the exe
RUN cp target/x86_64-pc-windows-gnu/release/live-ocr-translator.exe gtkapp

# copy gdbus.exe and required dlls
RUN cp gtklib/mingw64/bin/*.dll gtkapp
RUN cp gtklib/mingw64/bin/gdbus.exe gtkapp

# compile schemas 
RUN glib-compile-schemas gtklib/mingw64/share/glib-2.0/schemas

# copy remaining required files 
RUN cp gtklib/mingw64/share/glib-2.0/schemas/gschemas.compiled gtkapp/share/glib-2.0/schemas/gschemas.compiled \
    && cp -r gtklib/mingw64/share/icons/* gtkapp/share/icons \
    && cp -r gtklib/mingw64/lib/gdk-pixbuf-2.0 gtkapp/lib \
    && cp -r assets gtkapp

# copy the packaged app back to host
RUN mkdir gtkapp_v
RUN chmod +x copy.sh

VOLUME [ "app/gtkapp" ]

CMD ["./copy.sh"]