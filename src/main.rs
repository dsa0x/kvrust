pub mod kvrust;

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
        println!("{}", val);
    }
}
