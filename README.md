This project is an excercise for learning Rust and re-sharpening my programming skills.

Utilizes Floyd-Steinberg error diffusion with a weighted Euclidean distance formula to find near colors.

Some notes on the program: This program makes use of the image crate to open, create, and save images. However, all of the pixel editing happens outside of the image crate using the rgb crate.
I figured it would be easier to do math and other operations on the pixels using the rgb crate rather
than leaning on the image crate fully like I did in my previous program.

There is probably a way I could ONLY use a raw vector of u8 values, but converting everything into
rgb objects seems cleaner and easier to follow. I'm trying to make my code clean and precise rather than lightning fast.

USAGE:

dither2 [IMAGE FILE] [PALETTE FILE (optional]

EXAMPLES:

dither2 /home/User/Pictures/example.png /home/User/Downloads/palette.hex

dither2 ./picture.jpg