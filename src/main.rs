#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use image::{DynamicImage, GenericImageView, GenericImage};
use rand::random;
use time::Duration;

// Check if file exists
fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

// Check if file is of extension .png, .jpg, .jpeg
fn is_image(path: &str) -> bool {
    let ext = std::path::Path::new(path)
        .extension()
        .unwrap()
        .to_str()
        .unwrap()
        .to_lowercase();
    ext == "png" || ext == "jpg" || ext == "jpeg"
}

// Read file
fn read_image(path: &str) -> image::DynamicImage {
    image::open(path).unwrap()
}

// Write file
fn write_image(path: &str, img: &DynamicImage) {
    img.save(path).unwrap();
}

// Read user input informing the user of what to input
fn read_input(s: &str) -> String {
    println!("{}", s);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// Create a black image of dimensions width x height and save it to path
fn create_black_image(path: &str, width: u32, height: u32) {
    let img = image::DynamicImage::new_rgb8(width, height);
    write_image(path, &img);
}

// Generate N amount of random RGBA pixels
fn generate_random_pixels(n: u32) -> Vec<(u8, u8, u8, u8)> {
    (0..n).map(|_| {
        (
            random::<u8>(), // R
            random::<u8>(), // G
            random::<u8>(), // B
            random::<u8>(), // A
        )
    }).collect::<Vec<(u8, u8, u8, u8)>>()
}

// Calculate color distance between two pixels, as the sum of four differences squared
fn calculate_distance(p1: (u8, u8, u8, u8), p2: (u8, u8, u8, u8)) -> u32 {
    let r = (p1.0 ^ p2.0).pow(2);
    let g = (p1.1 ^ p2.1).pow(2);
    let b = (p1.2 ^ p2.2).pow(2);
    let a = (p1.3 ^ p2.3).pow(2);
    (r + g + b + a) as u32
}

// Find the closest pixel to a givel pixel out of a vector of pixels
fn find_closest_pixel(pixel: (u8, u8, u8, u8), pixels: &Vec<(u8, u8, u8, u8)>) -> [u8; 4] {
    if pixels.len() == 0 {
        return [0, 0, 0, 0]; // Return black pixel if there are no pixels
    }
    let mut closest_pixel = pixels[0];
    let mut closest_distance = calculate_distance(pixel, pixels[0]);

    for p in pixels {
        let distance = calculate_distance(pixel, *p);
        if distance < closest_distance  {
            closest_pixel = *p;
            closest_distance = distance;
        }
        // if distance < 50.0  {
        //     return [closest_pixel.0, closest_pixel.1, closest_pixel.2, closest_pixel.3]
        // }
    }
    [closest_pixel.0, closest_pixel.1, closest_pixel.2, closest_pixel.3]
}

// Read image and return a vector of pixels and their coordinates
fn read_image_pixels(path: &str) -> Vec<((u32, u32), (u8, u8, u8, u8))> {
    let mut pixels = Vec::new();
    // Read image
    let img = read_image(path);
    // Get image dimensions
    let width = img.width();
    let height = img.height();
    // Read each pixel
    for x in 0..width {
        for y in 0..height {
            let pixel = img.get_pixel(x, y);
            pixels.push(((x, y), (pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3])));
        }
    }
    pixels
}

// Calculate the average color of a vector of pixels
fn calculate_average_color(pixels: &Vec<(u8, u8, u8, u8)>) -> (u8, u8, u8, u8) {
    let mut r = 0;
    let mut g = 0;
    let mut b = 0;
    let mut a = 0;
    for p in pixels {
        r += p.0;
        g += p.1;
        b += p.2;
        a += p.3;
    }
    let n = pixels.len() as u8;
    (r / n, g / n, b / n, a / n)
}

// Color each pixel in the output image with the closest pixel in the input image
// the u32 in input_pixels is the x & y coordinates of the pixel in the input image
fn color_output_image(path: &str) {
    // Read input file
    let img = read_image(&path);

    // Create a black image of the same dimensions as the input image and return it, name it ${source_file}_output.png
    let output = format!("{}_output.{}", path.split(".").collect::<Vec<&str>>()[0], path.split(".").collect::<Vec<&str>>()[1]);
    // Create a black image if $output does not exist
    if !file_exists(&output) {
        create_black_image(&output, img.width(), img.height());
    } else {
        // Notify the user that the output image already exists
        println!("Output image already exists, it will be overwritten");
    }
    // Read the black image, which will be the output image
    let mut output_img = read_image(&output);

    // Generate N amount of random RGBA pixels, where N is the number of pixels in the input image
    let mut random_pixels = generate_random_pixels(img.width() * img.height());

    // For each pixel in the input image, find the closest pixel in the random pixels vector
    // and color the pixel in the output image with the closest pixel at the same coordinates

    let mut i = 0;
    let n = 2; // N x N pixels
    // Read the input image pixels
    let input_pixels = read_image_pixels(&path);

    // Define a step
    // Loop through the input image's nth pixels horizontally and vertically
    for x in 0..(img.width()/n) {
        for y in 0..(img.height()/n) {
            // Get the N pixels in a for loop
            let mut p = Vec::new();
            for k in 0..n {
                for l in 0..n {
                    let pixel = img.get_pixel(x * n + k, y * n + l);
                    p.push((pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3]));
                }
            }
            // Get the closest pixels
            let avg = calculate_average_color(&p);
            // Find the closest pixel to the average pixel
            let closest_pixel = find_closest_pixel(avg, &random_pixels);
            // Delete the closest pixel from the random pixels vector
            random_pixels.retain(|&x| x != (closest_pixel[0], closest_pixel[1], closest_pixel[2], closest_pixel[3]));
            // Color the output image pixel with the closest pixel
            for k in 0..n {
                for l in 0..n {
                    output_img.put_pixel(x * n + k, y * n + l, image::Rgba(closest_pixel));
                }
            }
            // Print progress as N^2
            i += n.pow(2);
            if i % 100 == 0 {
                println!("Progress: {} / {}", i, img.width() * img.height());
            }
        }
    }

    // Save the output image
    write_image(&output, &output_img);

    println!("Done");
    
}


// Main
fn main() {
    // Get input file path
    let input = read_input("Input file path:");
    // Start the timer
    let start = std::time::Instant::now();
    // Check if input file exists
    if !file_exists(&input) {
        println!("Input file does not exist");
        return;
    }
    // Check if input file is an image
    if !is_image(&input) {
        println!("Input file is not an image");
        return;
    }

    color_output_image(&input);

    // Stop the timer
    let duration = start.elapsed();
    // Print the time it took to run the program
    println!("Time elapsed: {:?}", duration);
}
