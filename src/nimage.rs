use image::GenericImageView;

fn open_image() {
    let img = image::open("files/part1.png").unwrap();
// The dimensions method returns the images width and height.
    println!("dimensions {:?}", img.dimensions());
// The color method returns the image's `ColorType`.
    println!("{:?}", img.color());
}

#[cfg(test)]
#[test]
fn test_opening() {
    open_image();
}