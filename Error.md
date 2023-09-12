enum CapError {
    Linux(LinError),
    Windows(WinError)
}

struct ImgCapError {
    Msg, 
    CapError
}

window_handle()                  
- Error : Fail to find window handle, window likely not open, win error code: X

window_rect()
- Error : Failed to get window rect, dimensions unkown, win error code : X.

monitor_sc()
	create_capture_item()
    - Error : Failed to get monitor dimensions, win error code : X

	take_sc()
		create_dynamic_image()
        - Error : Failed to create DynamicImage, 
            if error then:
                img = save_image()
                dynimg = dynamicimage_from_image(img)
            endif
	return -> DynamicImage


struct OCRError {
    Msg,
    TessErr
}

run_ocr_img()
	from_dynamic_image()
    - Error : Failed to create img from dynimg for OCR, TessErr code
	text_from_image()
    - Error : Failed to run ocr on image, TessErr code
	clean_text()
	return -> cleaned_text


struct TranslationError {
    Msg, 
    Reqwest_error
}

translate_text()
    - Error : Failed to translate text, reqwuest Err code
	returns -> translated_text

