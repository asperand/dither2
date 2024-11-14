use std::ops::Add;
use rgb::ComponentMap;
use std::ops::Sub;
use image::Rgb;
use image::ImageBuffer;
use std::path::Path;
use std::io::BufRead;
use std::fs::File;
use std::vec::Vec;
use std::io;
use rgb::RGB;
use image::open;

/// TODO: Add flags for choosing dithering or simple color replacement. -fs or -cr ?
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
    let mut image_tp = load_file(&file_path).expect("Couldn't open file"); // Get our tuple of the image sequence, height, and width.
    // let color_replaced_image = simple_color_replacement(&mut image_tp.0,user_palette); // perform a simple color replacement on our image
    let dithered_image = dither_image_fs(&mut image_tp.0,image_tp.2,user_palette);
    let new_raw = to_raw_from_rgb(dithered_image); // create a raw sequence of u8 from our object.
    let new_buffer: ImageBuffer<Rgb<u8>, _> =ImageBuffer::from_raw(image_tp.2,image_tp.1,new_raw).unwrap();
    let _ = match new_buffer.save("./dither.png") {
        Err(_) => println!("Couldn't save image buffer"),
        Ok(_) => println!("Saved image buffer to dither.png"),
    };
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

fn load_file(file_path : &String ) -> Result<(Vec<rgb::Rgb<u8>>, u32 ,u32),image::error::ImageError>{
    let mut image_rgb_vec = Vec::new();
    let image_file = open(file_path)?.to_rgb8();
    let image_height = image_file.height();
    let image_width = image_file.width(); // We need these as the raw sequence doesn't have h/w
    let image_raw = image_file.into_raw(); // Converting DynamicImage into a raw u8 sequence.
    for i in (0..image_raw.len()).step_by(3) { // For each 3 channel groupings, put them into a Vec.
        image_rgb_vec.push(RGB{r:image_raw[i],g:image_raw[i+1],b:image_raw[i+2]});
    }
    Ok((image_rgb_vec,image_height,image_width)) // Returning vec of invididual RGB values
}
/// Finds the nearest color in the given palette.
///
/// Uses a weighted Euclidean Distance formula in 3d...
/// 
/// sqrt((c1 - p1)^2 + (c2 - p1)^2 + (c2 - p1)^2)
///
/// With standard color weighting: r*0.3, g*0.59, b*0.11.

fn find_nearest_color(current_color:RGB<u8>,user_palette:Vec<RGB<u8>>) -> RGB<u8> {
    let mut lowest = 0;
    let mut max_distance = 441.672956; // max possible distance in a 256x256x256 box
    for i in 0..user_palette.len() {
        let eu_distance = 
            (((current_color.r as f32 - user_palette[i].r as f32) * 0.3).powi(2)
            +((current_color.g as f32 - user_palette[i].g as f32) * 0.59).powi(2)
            +((current_color.b as f32 - user_palette[i].b as f32) * 0.11).powi(2))
            .sqrt();
        if eu_distance < max_distance {
            max_distance = eu_distance;
            lowest = i;
        }        
    }
    return user_palette[lowest] // return our new color
}

/// The function that replaces colors of an image to their nearest palette pairing.
///
/// Uses find_nearest_color per pixel.

fn simple_color_replacement(image_rgb_vec:&mut Vec<RGB<u8>>,user_palette:Vec<RGB<u8>>) -> Vec<RGB<u8>> {
    for i in 0..image_rgb_vec.len(){
        image_rgb_vec[i] = find_nearest_color(image_rgb_vec[i],user_palette.clone());
    }
    return image_rgb_vec.to_vec()
}

/// This function iterates through each pixel of our image vector,
/// doing a basic color replacement and then diffusing the error throughout
/// the nearby pixels.
///
/// Has protection for wrapping on x+1 or x-1 pixels, but needs over/underflow protection
/// on addition and subtraction on RGB values.
/// 
fn dither_image_fs(image_rgb_vec:&mut Vec<RGB<u8>>,width:u32, user_palette:Vec<RGB<u8>>) -> Vec<RGB<u8>> {
    let mut wrapper_left = true;
    let mut wrapper_right = false;
    for i in 0..(image_rgb_vec.len()){ // For every pixel in the image
        let i_a = i as u32;
        let new_color = find_nearest_color(image_rgb_vec[i],user_palette.clone()); // find nearest color in palette
        
        // TODO: FIX OVERFLOWING ON SUB, LIKELY ON ADD TOO
        let quant_err = image_rgb_vec[i].sub(new_color); // quant error calc

        image_rgb_vec[(i_a+width) as usize] = image_rgb_vec[(i_a+width) as usize].add( // [x][y+1]
                quant_err.map(|p| (p as f32 * (0.3125)).round() as u8)); // 5/16
        if !wrapper_right { // if we are at the rightmost end
            image_rgb_vec[i+1] = image_rgb_vec[i+1].add( // [x+1],[y]
                quant_err.map(|p| (p as f32 * (0.4375)).round() as u8)); // 7/16
            image_rgb_vec[(i_a + (width+1)) as usize] = image_rgb_vec[(i_a + (width+1)) as usize].add( // [x+1][y+1]
                quant_err.map(|p| (p as f32 * (0.0625)).round() as u8)); // 1/16
        }
        if !wrapper_left{ // if we are at the leftmost end
            image_rgb_vec[(i_a + (width-1)) as usize] = image_rgb_vec[(i_a + (width-1)) as usize].add( // [x-1][y+1]
                quant_err.map(|p| (p as f32 * (0.1875)).round() as u8)); // 3/16
        }
        if (i_a+1) % width == 0{ // we are at the left starting next loop
            wrapper_left = true;
        }
        if (i_a+2) % width == 0{ // we are at the right starting next loop
            wrapper_right = true;
        }
    }
    return image_rgb_vec.to_vec()
}

/// Convert our vector of RGBs into a raw u8 sequence that the Image crate can work with.

fn to_raw_from_rgb(image_rgb_vec:Vec<RGB<u8>>) -> Vec<u8> {
    let mut raw_sequence = Vec::new();
    for pixel in image_rgb_vec {
        raw_sequence.push(pixel.r);
        raw_sequence.push(pixel.g);
        raw_sequence.push(pixel.b);
    }
    return raw_sequence
}