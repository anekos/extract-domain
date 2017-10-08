
extern crate publicsuffix;

use std::env::args;

use publicsuffix::{List, Host};

mod list;


fn main() {
    let list = List::from_string(list::LIST.to_owned()).unwrap();

    for arg in args().skip(1) {
        match list.parse_url(&arg) {
            Ok(Host::Domain(it)) => print_root(it.root()),
            _ => {
                match list.parse_domain(&arg) {
                    Ok(it) => print_root(it.root()),
                    _ => ()
                }
            }
        }
    }
}


fn print_root(root: Option<&str>) {
    if let Some(root) = root {
        println!("{}", root);
    }
}
