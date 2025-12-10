#[macro_use]
extern crate cvs_macro_lib;

use std::borrow::Cow;

fn main() {
    println!("Running cvs-macro-test-crate examples and tests...");
    test_cvs_macro();
}

fn test_cvs_macro() {
    let empty_list: Cow<'static, [Cow<'static, str>]> = cvs!();
    assert!(empty_list.is_empty());

    let single_item: Cow<'static, [Cow<'static, str>]> = cvs!("one");
    assert_eq!(single_item.len(), 1);
    assert_eq!(single_item[0], "one");

    let multiple_items: Cow<'static, [Cow<'static, str>]> = cvs!("apple", "banana", "cherry");
    assert_eq!(multiple_items.len(), 3);
    assert_eq!(multiple_items[0], "apple");
    assert_eq!(multiple_items[1], "banana");
    assert_eq!(multiple_items[2], "cherry");

    println!("cvs! macro tests passed!");
}

#[test]
fn it_works() {
    test_cvs_macro();
}
