use std::borrow::Borrow;
use std::ops::BitXor;

use image::{DynamicImage, GenericImageView, Pixel, PixelWithColorType, Rgb, Rgba};
use ndarray::Array2;
use rusqlite::types::Type;

pub struct Legend {
    pub colour: Rgb<u8>,
    pub grid: Vec<Vec<i64>>,
}

pub struct Cross {
    pub colour: Rgb<u8>,
    pub index: usize,
    pub x: u32,
    pub y: u32,
}


pub fn parse_square(img: &DynamicImage, legends: &Vec<Legend>, i: u32, j: u32) -> Cross {
    let size = legends.len();
    let mut deltas = vec![0; size];
    for xx in 0..19 {
        for yy in 0..29 {
            let mut line: String = "".to_owned();
            for item in img.get_pixel(47 + i * 33 + xx, 45 + j * 33 + yy).to_rgb().channels().iter() {
                line.push_str(&item.to_string().to_owned());
            }

            for del_ind in 0..size {
                let item = (legends[del_ind].grid[xx as usize][yy as usize] - line.parse::<i64>().unwrap()).abs();
                deltas[del_ind] = deltas[del_ind] + (item as usize);
            }
        }
    }
    let mut min = deltas[0];
    let mut ind = 0;
    for del_ind in 1..size {
        if min > deltas[del_ind] {
            min = deltas[del_ind];
            ind = del_ind;
        }
    }
    Cross {
        colour: legends[ind].colour,
        index: ind,
        x: i,
        y: j,
    }
}

pub fn schema_dimensions(img: &DynamicImage) -> (u32, u32) {
    let ir = (img.dimensions().0 - 44) / 33;
    let jr = (img.dimensions().1 - 44) / 33;
    (ir, jr)
}

pub fn open_image(filePath: &str) -> DynamicImage {
    image::open(filePath).unwrap()
}

pub fn parse_legend(img: &DynamicImage) -> Vec<Legend> {
    //legend structure:x 0(6 items) - 480(6 items)  - 959(5 items) ; y - from 25
    let start6: [u32; 2] = [0, 481];
    let start5: u32 = 960;
    let mut res: Vec<Legend> = Vec::new();
    for start in start6.iter() {
        for row in 0..6 {
            res.push(extract_legend_item(img, start, &row));
        }
    }
    for row in 0..5 {
        res.push(extract_legend_item(img, &start5, &row));
    }
    res
}

fn extract_legend_item(img: &DynamicImage, start: &u32, row: &u32) -> Legend {
    let mut legend: Legend = Legend { colour: Rgb([0, 0, 0]), grid: vec![vec![0; 30]; 24] };
    legend.colour = img.get_pixel(start + 35, 25 + 33 * row).to_rgb();
    for xx in 0..19 {
        for yy in 0..29 {
            let mut line: String = "".to_owned();
            for item in img.get_pixel(start + xx, 25 + 33 * row + yy).to_rgb().channels().iter() {
                let line2 = item.to_string().to_owned();
                line.push_str(&line2);
            }
            legend.grid[xx as usize][yy as usize] = line.parse::<i64>().unwrap();
        }
    }
    legend
}


#[cfg(test)]
#[test]
fn test_legend() {
    open_image("files/legend.png");
}

#[test]
fn test_opening() {
    open_image("files/part11.png"); // not panicking - good
}

#[test]
fn test_parse_legend() {
    let legends = parse_legend(&open_image("files/legend.png"));
    assert_eq!(17, legends.len());
    let cross = parse_square(&open_image("files/part11.png"), &legends, 1, 1);
    assert_eq!(0, cross.index);
}