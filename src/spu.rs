#[allow(
    dead_code,
    unused_crate_dependencies,
    unused_variables,
    unused_assignments,
    unused,
    unused_imports,
    non_snake_case,
    non_camel_case_types,
    warnings
)]
use base64::{decode, encode};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat, ImageOutputFormat};
use std::fs::File;
use std::io::Error;
use std::io::Read;
use std::io::Write;

use reqwest::Client;
use tokio::runtime::Runtime;
use tokio::time::{self, Duration};

pub fn _encode_file_to_base64(file_path: &str, param1: u32, param2: u32) -> String {
    // logic to encode the file to base64
    // read the image file
    let image = image::open(file_path).expect("Failed to open image");
    // Reduce the size of the image by 1/4
    let resized_image = image.resize(
        image.width() / param1,
        image.height() / param2,
        FilterType::Lanczos3,
    );
    let mut compressed_image_data = Vec::new();
    resized_image
        .write_to(&mut compressed_image_data, ImageFormat::Jpeg)
        .expect("Failed to compress image");
    // Encode the compressed image to base64
    let base64_compressed_image = encode(&compressed_image_data);
    base64_compressed_image
}

pub fn _encode_image_fromURLToBase64(image_url: &str, param1: u32, param2: u32) -> String {
    let mut rt = Runtime::new().unwrap();

    let result = rt.block_on(async {
        let client = Client::new();
        let response = client.get(image_url).send().await.unwrap();
        let buf = response.bytes().await.unwrap();

        let image = image::load_from_memory(&buf).unwrap();
        // Reduce the size of the image by 1/4
        let resized_image = image.resize(
            image.width() / param1,
            image.height() / param2,
            FilterType::Lanczos3,
        );

        let mut encoded = Vec::new();
        resized_image
            .write_to(&mut encoded, ImageOutputFormat::Png)
            .unwrap();

        base64::encode(&encoded)
    });

    result
}
/*
pub fn writeBase64ToFile(filename_ext: String, base64_text: String) {
    // Create a new file to save the base64 encoded image
    //let filename_ext = "base64_image.txt";
    let mut base64_output_file = File::create(filename_ext.clone()).expect("Failed to create file");
    // Save the base64 encoded image to file
    base64_output_file
        .write_all(base64_text.as_bytes())
        .expect("Failed to write base64 image to file");

    println!("Base64text saved as {} to file", &filename_ext);
}

fn base64_to_image(filename: &str) {
    let mut file = File::open(filename).unwrap();
    let mut encoded_string = String::new();
    file.read_to_string(&mut encoded_string).unwrap();

    // let decoded_image: Vec<u8> = base64::decode(&encoded_string).unwrap();
    // let image = ImageBuffer::from_raw(
    //     decoded_image.len() as u32,
    //     decoded_image.len() as u32,
    //     decoded_image,
    // )
    // .unwrap();
    // image.save("image.png").unwrap()

    // Decode the Base64 string
    let decoded_data = decode(encoded_string).expect("Failed to decode Base64 string");

    // Create a new file and write the decoded data to it
    let mut file = File::create("image.png").expect("Failed to create file");
    file.write(&decoded_data).expect("Failed to write to file");
}

*/
