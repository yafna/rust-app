use pdf::file::File;

macro_rules! run {
    ($e:expr) => (
        match $e {
            Ok(v) => v,
            Err(e) => {
                e.trace();
                panic!("{}", e);
            }
        }
    )
}

pub fn read_pages(filename: &str) -> i32 {
    let file = run!(File::<Vec<u8>>::open(filename));
    let mut num = 0;
    for i in 0..file.num_pages() {
        println!("Read page {}", i);
        let _ = file.get_page(i);
        num += 1;
    }
    num
}

#[cfg(test)]
#[test]
fn pdf_open() {
    let item = read_pages("files/zergling.pdf");
    assert_eq!(8, item);
}