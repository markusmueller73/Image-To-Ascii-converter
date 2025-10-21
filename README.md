# Image-To-Ascii-converter
A console application, that converts images in various formats to ascii char 'images'

## Dependencies
For image loading and saving (for debug purposes) it used the rust crate [image(https://github.com/image-rs/image)].

## Image manipulation
The functions for resizing, dithering and greyscale conversion made be myself. Especially for dithering I read the [excellent article(https://tannerhelland.com/2012/12/28/dithering-eleven-algorithms-source-code.html)] from Tanner Helland. Self teaching about the resizing algorisms starts in the WIkipedia and follows the links from there. All over the net are many useful explanaitions about scaling an image.

## Usage

`img2asc <FILE> [OPTIONS]`

### Options:

`-a <TYPE>       --ascii <TYPE>          type of ascii char set`  
`-f <FILE>       --filename <FILE>       path and filename from the image file`  
`-g <TYPE>       --greyscale <TYPE>      the greyscale conversion algorithm`  
`-h <NUM>        --height <NUM>          the height of the ascii image`  
                `--help                  show this help text`  
`-i              --invert                invert the image colors`  
`-r <TYPE>       --resize <TYPE>         the resize algorithm`  
`-t <NUM>        --threshold <NUM>       the threshold from black (0) to white (255)`  
`-V              --version               the version of img2asc`  
`-w <NUM>        --width <NUM>           the width of the ascii image`  

