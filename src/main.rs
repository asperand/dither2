use image::ImageReader;
use image::ImageBuffer;
use std::path::Path;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;
use std::io;
use rgb::RGB;
use image::open;

fn main() {
    let _args = std::env::args();
    let file_path = std::env::args().nth(1).expect("No image file provided");
    let palette_path = match std::env::args().nth(2) { // Check if palette file was provided.
        Some(palette_path) => palette_path,
        None => " ".to_string(),
    };
    let user_palette_result = setup_palette(palette_path);
    let user_palette = match user_palette_result { // Handling for no file found.
        Ok(user_palette) => user_palette,
        Err(_) => vec![RGB{r:3,g:3,b:3},RGB{r:255,g:255,b:255}],
    };
    let image_rgb_vec = load_file(&file_path).expect("Couldn't open file");
    let color_replaced_image = color_replacement(image_rgb_vec,user_palette);
}


/// Sets up the color palette for color reduction in the image.
///
/// Expects hexcode values written in 6 characters with no
/// other formatting such as '0x' or '#'. Will skip bad
/// lines, but will not fix them.
///
/// # Example of a good palette file:
///
/// colors.hex
/// --------------------------------
/// 00FFFF
/// FFFFFF
/// 000000
/// FF00FF
/// --------------------------------

fn setup_palette<P>(palette_path: P) -> Result<Vec<RGB<u8>>,std::io::Error>
where P: AsRef<Path>, {
    let mut new_color = Vec::new();
    let mut user_palette = Vec::new();
    let file = File::open(palette_path)?;
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten(){
        if line.chars().any(|c| c.is_ascii_hexdigit()) == false || line.len() != 6 {
            continue; // Skip the current line if it doesn't meet our standards.
        }
        let mut cur = line.clone();
        while !cur.is_empty(){ // Recursive sub-string splitting.
            let (color, rest) = cur.split_at(2);
            new_color.push(u8::from_str_radix(color,16).unwrap());
            cur = rest.to_string();
        }
        let pal_rgb = RGB {r:new_color[0], g:new_color[1], b:new_color[2]};
        user_palette.push(pal_rgb);
        new_color.clear();
    }
    if user_palette.is_empty(){ // Only fires if there were no valid colors in the file.
        user_palette.push(RGB{r:3,g:3,b:3});
        user_palette.push(RGB{r:255,g:255,b:255});
        println!("No valid colors found, default palette will be used.");
    }
    Ok(user_palette)
}

/// Load our image file and turn it into a RAW Vec<u8>,
/// and then turn it into a vector of RGB pixels.
///
/// This is not to be mistaken with image::Rgb!
/// This program uses rgb::RGB pixels so we can actually
/// do math on them through rgb's functions.

fn load_file(file_path : &String ) -> Result<Vec<rgb::Rgb<u8>>,image::error::ImageError>{
    let mut image_rgb_vec = Vec::new();
    let image_file = open(file_path)?.to_rgb8().into_raw(); // Converting DynamicImage into a raw u8 sequence.
    for i in (0..image_file.len()).step_by(3) { // For each 3 channel groupings, put them into a Vec.
        image_rgb_vec.push(RGB{r:image_file[i],g:image_file[i+1],b:image_file[i+2]});
    }
    Ok(image_rgb_vec)
}
/// Finds the nearest color in the given palette.
///
/// Uses a weighted Euclidean Distance formula in 3d...
/// 
/// sqrt((c1 - p1)^2 + (c2 - p1)^2 + (c2 - p1)^2)
///
/// With standard color weighting: r*0.3, g*0.59, b*0.11.

fn find_nearest_color(current_color:RGB<u8>,user_palette:Vec<RGB<u8>>){



}

/// The function that replaces colors of an image to their nearest palette pairing.
///
/// First converts the PhotonImage into raw pixels (a vector of u8 values)
/// Then, iterates over and parses out those pixels, and sends them to our
/// find_nearest_color() function, which then returns a brand new pixel.
/// Then, we replace the previous values with the new pixel values.
///
/// Repeat ad nausem

fn color_replacement(image_rgb_vec:Vec<RGB<u8>>,user_palette:Vec<RGB<u8>>){
 // stepping by four to group pixel.
        /* PSEUDOCODE
            
            GRAB CURRENT PIXEL VALUES FROM RAW_IMAGE
            PASS TO FIND_NEAREST_COLOR, WHICH RETURNS A SINGLE COLOR

            1. EDIT RAW_IMAGE AND REPLACE VALUES

            2. CREATE A NEW RAW IMAGE VECTOR



        */

    // FINALLY, CREATE NEW IMAGE FROM NEW RAW DATA
}
