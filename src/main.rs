use std::path::Path;
use std::io::BufRead;
use std::fs::File;
use std::env;
use std::vec::Vec;
use std::io;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let palette_path = &args[2];
    println!("{}", file_path);
    println!("{}", palette_path);
    let mut user_palette = Vec::new();
    if let Ok(lines) = setup_palette(palette_path){
        for line in lines.flatten(){
            user_palette.push(i32::from_str_radix(&line,16).expect("Couldn't read hex values from file."))
        }
    }
    else{ // If the user file doesn't exist or can't be loaded, create a default monochrome palette.
        user_palette.push(0xFFFFFF);
        user_palette.push(0x000000);
    }
    let image_file = load_file(file_path);
    let color_replaced_image = color_replacement(image_file,user_palette);
}


/// Sets up the color palette for color reduction in the image.
///
/// Takes in any file, but expects hexcode values.
/// Will ignore any "junk" found in the file, but does minimal
/// checking otherwise. It is up to the user to provide a clean
/// color palette.
///
/// # Example of a good palette file:
///
/// colors.hex ()
/// --------------------------------
/// 00FFFF
/// FFFFFF
/// 000000
/// FF00FF
/// --------------------------------

fn setup_palette<P>(file_path: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(file_path)?;
    Ok(io::BufReader::new(file).lines())
}

/// Load our image file using photon.

fn load_file(file_path:&String){
    
}

/// Finds the nearest color in the given palette.
///
/// Uses a weighted Euclidean Distance formula in 3d...
/// 
/// sqrt((c1 - p1)^2 + (c2 - p1)^2 + (c2 - p1)^2)
///
/// With standard color weighting: r*0.3, g*0.59, b*0.11.

fn find_nearest_color(current_color:Vec<u8>,user_palette:Vec<i32>){



}

/// The function that replaces colors of an image to their nearest palette pairing.
///
/// First converts the PhotonImage into raw pixels (a vector of u8 values)
/// Then, iterates over and parses out those pixels, and sends them to our
/// find_nearest_color() function, which then returns a brand new pixel.
/// Then, we replace the previous values with the new pixel values.
///
/// Repeat ad nausem

fn color_replacement(image_file:(),user_palette:Vec<i32>){
 // stepping by four to group pixel.
        /* PSEUDOCODE
            
            GRAB CURRENT PIXEL VALUES FROM RAW_IMAGE
            PASS TO FIND_NEAREST_COLOR, WHICH RETURNS A SINGLE COLOR

            1. EDIT RAW_IMAGE AND REPLACE VALUES

            2. CREATE A NEW RAW IMAGE VECTOR



        */

    // FINALLY, CREATE NEW IMAGE FROM NEW RAW DATA
}
