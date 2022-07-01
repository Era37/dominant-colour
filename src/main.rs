mod utils;
use image::io::Reader;
use std::collections::HashMap;
use utils::{find_color_from_size, find_largest_entry, rgb_to_hex};

fn avg_color(buffer: Vec<u8>) -> String {
    let mut map: HashMap<Vec<u8>, i32> = HashMap::new();
    let mut image_vector: Vec<Vec<u8>> = Vec::new();
    for i in buffer.chunks_exact(3) {
        image_vector.push(i.to_vec());
    }
    for pixel in image_vector {
        if pixel == [0, 0, 0] {
            continue;
        }
        let val = map.get_mut(&pixel);
        if val.is_some() {
            let increased_value = val.unwrap().clone() + 1;
            map.insert(pixel, increased_value);
        } else {
            map.insert(pixel, 1);
        }
    }
    let largest_value: i32 = find_largest_entry(&map);
    let rgb_value: Vec<u8> = find_color_from_size(largest_value, &map);
    rgb_to_hex(rgb_value)
}
fn main() {
    let img = Reader::open("image.jpg").unwrap().decode().unwrap();
    let c = avg_color(img.to_rgb8().to_vec());
    println!("{}", c);
}
