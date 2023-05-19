#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use image::{DynamicImage, GenericImageView, GenericImage};
use rand::random;

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
    let mut pixels = Vec::new();
    for _ in 0..n {
        let r = rand::random::<u8>();
        let g = rand::random::<u8>();
        let b = rand::random::<u8>();
        let a = rand::random::<u8>();
        pixels.push((r, g, b, a));
    }
    pixels
}

// Calculate color distance between two pixels, as the square root of the sum of four differences raised to the second power
fn calculate_distance(p1: (u8, u8, u8, u8), p2: (u8, u8, u8, u8)) -> f64 {
    let r = (p1.0 as f64 - p2.0 as f64).powf(2.0);
    let g = (p1.1 as f64 - p2.1 as f64).powf(2.0);
    let b = (p1.2 as f64 - p2.2 as f64).powf(2.0);
    let a = (p1.3 as f64 - p2.3 as f64).powf(2.0);
    (r + g + b + a).sqrt()
}

// Find the closest pixel to a givel pixel out of a vector of pixels
fn find_closest_pixel(pixel: (u8, u8, u8, u8), pixels: &Vec<(u8, u8, u8, u8)>) -> [u8; 4] {
    let mut closest_pixel = pixels[0];
    let mut closest_distance = calculate_distance(pixel, pixels[0]);
    for p in pixels {
        let distance = calculate_distance(pixel, *p);
        if distance < closest_distance  {
            closest_pixel = *p;
            closest_distance = distance;
        }
        if distance < 50.0  {
            return [closest_pixel.0, closest_pixel.1, closest_pixel.2, closest_pixel.3]
        }
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

    // Generate N amount of random RGBA pixels, where N is the number of pixels in the input image
    let mut pixels = generate_random_pixels(img.width() * img.height());

    // Create a black image of the same dimensions as the input image and return it
    let output = "samples/output.png";
    create_black_image(output, img.width(), img.height());
    // Read the black image, which will be the output image
    let mut output_img = read_image(output);

    // Read the input image pixels
    let input_pixels = read_image_pixels(&path);

    // For each pixel in the input image, find the closest pixel in the random pixels vector
    // and color the pixel in the output image with the closest pixel at the same coordinates
    let n: u32 = 1;
    let mut i = 0;

    // Check if n is equal to 1
    if n == 1 {
        // Sort the pixels by their red, green, blue, and alpha values
        pixels.sort_by(|a, b| a.0.cmp(&b.0));
        // Sort the input image pixels by their red, green, blue, and alpha values, which are in the second tuple
        let mut input_pixels2 = input_pixels.clone();
        input_pixels2.sort_by(|a, b| a.1.cmp(&b.1));
        // For each pixel in the input image, find the closest pixel in the random pixels vector
        // and color the pixel in the output image with the closest pixel at the same coordinates
        for (i, pixel) in input_pixels2.iter().enumerate() {
            // Get the closest pixel
            let closest_pixel = find_closest_pixel(pixel.1, &pixels);
            // Create a copy of closest_pixel as (u8, u8, u8, u8)
            let closest_pixel2 = (closest_pixel[0], closest_pixel[1], closest_pixel[2], closest_pixel[3]);
            // Delete the closest pixel from the random pixels vector
            pixels.retain(|&x| x != closest_pixel2);
            // Color the output image pixel with the closest pixel
            output_img.put_pixel(pixel.0 .0, pixel.0 .1, image::Rgba(closest_pixel));
            // Print progress as pixels are colored
            println!("Progress: {} / {}", i, img.width() * img.height());
        }
    } else {
        for x in (0..(img.width()/n)-1).step_by(n as usize) {
            for y in (0..(img.height()/n) - 1).step_by(n as usize) {
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
                let closest_pixel = find_closest_pixel(avg, &pixels);
                // Create a copy of closest_pixel as (u8, u8, u8, u8)
                let closest_pixel2 = (closest_pixel[0], closest_pixel[1], closest_pixel[2], closest_pixel[3]);
                // Delete the closest pixel from the random pixels vector
                pixels.retain(|&x| x != closest_pixel2);
                // Color the output image pixel with the closest pixel
                for k in 0..n {
                    for l in 0..n {
                        output_img.put_pixel(x * n + k, y * n + l, image::Rgba(closest_pixel));
                    }
                }
                // Print progress as N * N pixels are colored
                i += n * n;
                println!("Progress: {} / {}", i, img.width() * img.height());
            }
        }
    }

    // Save the output image
    write_image(output, &output_img);

    println!("Done");
    
}


// Main
fn main() {
    // Get input file path
    let input = read_input("Input file path:");
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

    color_output_image(&input)

}
