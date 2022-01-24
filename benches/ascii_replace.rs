use iai::{black_box, main};


pub fn safe_replace(s: &mut str) {
    let mut m = in_place_string_map::MapInPlace::new(s);
    while !m.unmapped().is_empty() {
        if m.unmapped().as_bytes()[0] == b'\\' {
            m.pop_chars(1).unwrap();
            m.push_str("/").unwrap();
        } else {
            m.move_chars(1).unwrap();
        }
    }
}

pub fn unsafe_replace(s: &mut str) {
    unsafe {
        let b = s.as_bytes_mut();
        for c in b {
            if *c == b'\\' {
                *c = b'/';
            }
        }
    }
}

fn safe_replace_bench() -> String {
    let mut s = black_box("hello\\world".to_owned());
    safe_replace(&mut s);
    s
}

fn unsafe_replace_bench() -> String {
    let mut s = black_box("hello\\world".to_owned());
    unsafe_replace(&mut s);
    s
}


main!(safe_replace_bench, unsafe_replace_bench);
