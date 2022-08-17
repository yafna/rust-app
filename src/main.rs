use std::borrow::Borrow;
use rusqlite::Connection;
use crate::nimage::Legend;

mod nimage;
mod ndb;

fn main() {
    let connection = ndb::connect();
    let legends = nimage::parse_legend(&nimage::open_image("files/legend.png"));
    for ind in 0..legends.len() {
        let mut i = 0;
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for item in legends[ind].colour.0 {
            if i == 2 {
                b = item
            }
            if i == 1 {
                g = item;
                i = 2;
            }
            if i == 0 {
                r = item;
                i = 1;
            }
        }
        ndb::insert_legend(&connection, ind, r, g, b).expect("Failed to insert legend");
        for xx in 0..19 {
            for yy in 0..29 {
                ndb::insert_grid(&connection, ind,
                                 legends[ind].grid[xx as usize][yy as usize], xx, yy).expect("Failed to insert legend grid");
            }
        }
    }
    parse_img("files/part11.png", &connection, &legends,1, 1);
    parse_img("files/part12.png", &connection, &legends,61, 1);
    parse_img("files/part13.png", &connection, &legends,121, 1);
    parse_img("files/part21.png",&connection, &legends,1, 91);
    parse_img("files/part22.png", &connection, &legends,61, 91);
    parse_img("files/part23.png", &connection, &legends,121, 91);
}
fn parse_img(file_path: &str, conn: &Connection, legends: &Vec<Legend>, x: u32, y:u32){
    let img = nimage::open_image(file_path);
    let dimensions = nimage::schema_dimensions(&img);
     for i in 0..dimensions.0 {
        for j in 0..dimensions.1 {
            let cross = nimage::parse_square(&img, &legends, i, j);
            ndb::insert_square_data(&conn, cross.index, i + x, j + y).expect("Failed");
        }
    }
}