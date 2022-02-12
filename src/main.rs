pub mod kvrust;
mod strct;
use strct::*;

fn main() {
    let mut kv: kvrust::Kvrust<&str, &str> = kvrust::Kvrust::new();

    let values: Vec<(&str, &str)> = vec![
        ("key1", "val1"),
        ("key2", "val2"),
        ("key3", "val4"),
        ("key3", "val3"),
        ("key5", "val5"),
    ];

    for v in &values {
        kv.add(v.0, v.1);
    }
    for v in values {
        let val = kv.get(v.0);
        print!("{} ", val);
    }
    println!();

    let new_test_str = test();

    let new_str = TestStr::new(50);

    println!("Is younger: {:?}", new_str.is_younger(new_test_str))
}
