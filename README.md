# live-ocr-translator

![Static Badge](https://img.shields.io/badge/OCR-tesseract-blue)
![Static Badge](https://img.shields.io/badge/GUI-gtk-blue)

## Why

This is basically just a desktop implementation of googles version of live translation. To my knowledge the three main options for translating on screen content are

- Screenshot
- Google Translate app live translate
- Copy pasting text

This project aims to add a 4th option where you can specify a region of text on the screen to translate and then either translate just any text recognized in this specified segment or translate the text in real time as it changes.

### Applications

- Live translation of subtitles when preferred option does not exist
- Live translation of text content in games without a translation
- General translation of any text visible

## Features

- [ ] Basic image translation for common languages
- [ ] Live translation for changing text
- [ ] Easy to use GUI built with gtk
- [ ] Accurate translation through image processing
- [ ] Sound to text system

## Contributing

Currently I am developing primarily for windows so to test the application on windows first build the dockerfile then each time you want to compile the application for windows run the `cross_compile.sh` script.

To build image with the required dependencies

```bash
docker build . -t gtkrs-cross
```

To cross-compile the application for windows

```bash
./cross_compile.sh
```

The packaged application should then appear in your root directory in the folder gtkapp

### Visual feature outline

![image](assets/feature_outline.png)
