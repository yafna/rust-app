use std::any::*;

use pdf::any::AnyObject;
use pdf::backend::Backend;
use pdf::file::*;
use pdf::object::*;

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

//.. we know, that :
/// gen 0 :  id : 1 --- msg: "expected pdf::object::stream::ObjectStream, found pdf::object::types::PagesNode"
//           id : 42 --- msg: "expected pdf::object::stream::ObjectStream, found pdf::object::types::Catalog"

pub fn get_catalog(filename: &str, id_pages: u64) -> RcRef<Catalog> {
    let file = run!(File::<Vec<u8>>::open(filename));
    let res : RcRef<Catalog> = run!(file.get(Ref::new(PlainRef { id: id_pages, gen: 0})));
    res
}

pub fn get_pages_node(filename: &str, id_pages: u64) -> RcRef<PagesNode> {
    let file = run!(File::<Vec<u8>>::open(filename));
    let res : RcRef<PagesNode> = run!(file.get(Ref::new(PlainRef { id: id_pages, gen: 0})));
    res
}

pub fn scan_objects(filename: &str) {
    let file = run!(File::<Vec<u8>>::open(filename));
    for i in 0..150 {
        println!(" {} --- ", i);
        let res : Result<RcRef<ObjectStream>, _> = file.get(Ref::new(PlainRef { id: i, gen: 0}));
        match res {
            Ok(v) => { println!("fff {}", i) }
            Err(e) => { println!("error occured {e:?}"); }
        };
    }
}

pub fn num_pages(filename: &str) -> i32 {
    let file = run!(File::<Vec<u8>>::open(filename));
    let mut num = 0;
    for i in 0..file.num_pages() {
        let _ = file.get_page(i);
        num += 1;
    }
    num
}

#[cfg(test)]
// #[test]
// fn scan_objects() {
//     find_object_stream("files/zergling.pdf");
// }

#[test]
fn test_catalog() {
    let catalog = get_catalog("files/zergling.pdf", 42);
    println!("pages count {}",catalog.pages.count);
}

#[test]
fn pdf_open() {
    let item = num_pages("files/zergling.pdf");
    assert_eq!(8, item);
}