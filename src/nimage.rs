use std::borrow::Borrow;
use std::ops::BitXor;

use image::{DynamicImage, GenericImageView, Pixel, Rgba};

struct Square {
    nhash: i32,
    count: i32,
}

fn parse_square(img: DynamicImage, i: u32, j: u32) -> i64 {
    let mut gg = 0;
    for xx in 0..30 {
        for yy in 0..30 {
            let mut line: String = "".to_owned();
            for item in img.get_pixel(44 + i * 33 + xx, 44 + j * 33 + yy).to_rgb().channels().iter() {
                let line2 = item.to_string().to_owned();
                line.push_str(&line2);
            }
            let dd = line.parse::<i64>().unwrap();
            gg = gg ^ dd;
        }
    }
    gg
}

fn open_image() -> DynamicImage {
    image::open("files/part1.png").unwrap()
}

#[cfg(test)]
#[test]
fn test_opening() {
    open_image(); // not panicking - good
}

#[test]
fn test_parse_image() {
    // let ir = (img.dimensions().0 - 44) / 33;
    // let jr = (img.dimensions().1 - 44) / 33;
    assert_eq!(254555242, parse_square(open_image(), 1, 1));
}