use std::borrow::Borrow;

mod nimage;
mod ndb;

fn main() {
    let legends = nimage::parse_legend(&nimage::open_image("files/legend.png"));
    let img = nimage::open_image("files/part1.png");
    let dimensions = nimage::schema_dimensions(&img);
    let xcol: u32 = dimensions.0;
    let ycol: u32 = dimensions.1;

    for i in 0..xcol {
        for j in 0..ycol {
            let cross = nimage::parse_square(&img, &legends, i, j);
            print!(" {}", cross.index);
        }
        println!();
    }
}