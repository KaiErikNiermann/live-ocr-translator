FROM rust:latest

WORKDIR /app 

COPY build.sh .

# install mingw64 for link 
RUN rustup toolchain install stable-x86_64-pc-windows-gnu
RUN rustup update

# make the directories for the app and its dependencies
RUN mkdir -p gtklib \
    && mkdir -p gtkapp/share/glib-2.0/schemas/ \
    && mkdir -p gtkapp/share/icons/ \
    && mkdir -p gtkapp/lib/gdk-pixbuf-2.0/ 

# get the required mingw libs 
RUN wget https://github.com/qarmin/gtk_library_store/releases/download/3.24.0/mingw64.zip
RUN unzip mingw64.zip -d gtklib

# add windows target 
RUN rustup target add x86_64-pc-windows-gnu

# get the linker for windows
RUN apt-get update && apt-get upgrade -y
RUN apt-get install gcc-mingw-w64-x86-64 -y

# download tesseract-ocr and add to gtkapp
RUN apt install python3-pip -y \
    && pip3 install gdown --break-system-packages \
    && gdown --fuzzy https://drive.google.com/file/d/1ipXvr6M9fYcGde7263WQ10aZpbhssvmC/view?usp=drive_link \
    && unzip tesseract-ocr.zip -d .

RUN chmod +x build.sh

VOLUME [ "app" ]

CMD ["./build.sh"]