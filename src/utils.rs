use colors_transform::Rgb;
use std::collections::HashMap;

pub fn find_largest_entry(map: &HashMap<Vec<u8>, i32>) -> i32 {
    let mut all_values: Vec<i32> = Vec::new();
    for value in map {
        all_values.push(*value.1);
    }
    let mut largest_value: i32 = 0;
    for num in all_values {
        if num > largest_value {
            largest_value = num;
        }
    }
    largest_value
}

pub fn find_color_from_size(size: i32, map: &HashMap<Vec<u8>, i32>) -> Vec<u8> {
    let mut color: &Vec<u8> = &Vec::new();
    for v in map {
        if v.1 == &size {
            color = v.0
        }
    }
    color.to_owned()
}

pub fn rgb_to_hex(rgb: Vec<u8>) -> String {
    let mut rgb_convert: Vec<f32> = Vec::new();
    for c in rgb {
        rgb_convert.push(f32::from(c));
    }
    let str: String = Rgb::from(rgb_convert[0], rgb_convert[1], rgb_convert[2]).to_css_hex_string();
    str.to_ascii_uppercase()
}
