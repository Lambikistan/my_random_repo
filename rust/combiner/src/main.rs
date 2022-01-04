// Note: from https://www.freecodecamp.org/news/rust-in-replit/#rust-overview
mod args;

use args::Args;
use image::{ io::Reader, DynamicImage, ImageFormat, GenericImageView, imageops::FilterType::Triangle };

fn main() -> Result<(), ImageDataErrors>  {
    let args = Args::new();
    println!("{:?}", args);

    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);

    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    }

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);
    // ? is shorthand for propogate error if error returned
    output.set_data(combined_data)?;

    // Now save the file using image format
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
        ).unwrap();
    
    // return empty tuple
    Ok(())
}

// Debug trait can be implemented using attributes (derived)
#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}

impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = 3_655_744;
        let buffer: Vec<u8> = Vec::with_capacity(buffer_capacity);
        FloatingImage {
            width,
            height,
            data: buffer,
            name,
        }
    }

    // Method within the struct requires self
    fn set_data(&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity() {
            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;

        Ok(())
    }
}

fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();

    (image, image_format)
}

fn get_smallest_dimensions(dim_1: (u32, u32), dim_2: (u32, u32)) -> (u32, u32) {
    // access tuples using dot (.) notation to index individual tuple elements
    let pix_1= dim_1.0 * dim_1.1;
    let pix_2= dim_2.0 * dim_2.1;

    return if pix_1 < pix_2 { dim_1 } else { dim_2 };
}

fn standardise_size(image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width_1, height_1) = image_1.dimensions();
    let (width_2, height_2) = image_2.dimensions();
    let (width, height) = get_smallest_dimensions(image_1.dimensions(), image_2.dimensions());
    println!("img1 width, height: {},{}", width_1, height_1);
    println!("img2 width, height: {},{}", width_2, height_2);
    println!("std  width, height: {},{}", width, height);

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }
}

fn combine_images( image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgba8().into_vec();
    let vec_2 = image_2.to_rgba8().into_vec();

    alternate_pixels(vec_1, vec_2)
}

fn alternate_pixels( vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8> {
    // Create vector of size vec_1 length
    let mut combined_data = vec![0u8; vec_1.len()];

    // Using a while loop requires that you declare the loop var outside?
    let mut i =0;
    // TODO: Why 8, 3, and 4?  
    while i < vec_1.len() {
        if i % 8 == 0 {
            // Range syntax for inclusive is ..=
            combined_data.splice(i..=i + 3, set_rgba(&vec_1, i, i + 3));
        } else {
            combined_data.splice(i..=i + 3, set_rgba(&vec_2, i, i + 3));
        }
        i += 4;
    }

    combined_data
}

fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {
    let mut rgba = Vec::new();
    for i in start..=end {
        let val = match vec.get(i) {
            Some(d) => *d,
            None => panic!("Index out of bounds"),
        };
        rgba.push(val);
    }

    rgba
}
