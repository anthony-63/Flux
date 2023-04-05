use std::io::{BufReader, Cursor, BufRead, Read};

pub fn convert_sspm1(data:Vec<u8>) -> Vec<u8> {
    let ret = Vec::new();
    let mut r =BufReader::new(Cursor::new(data));
    let mut _dnc = String::new();
    r.read_line(&mut _dnc);
    r.read_line(&mut _dnc);
    r.read_line(&mut _dnc);
    print!("{_dnc}");
    let mut note_count = [0;4];
    r.read_exact(&mut [0;4]);
    r.read_exact(&mut note_count);
    let mut diff = [0;1];
    r.read_exact(&mut diff);
    r.read_exact(&mut [0;1]);
    let mut img_type = [0;1];
    r.read_exact(&mut img_type);
    // r.read

    println!("{note_count:?} {diff:?} {img_type:?}");

    ret
}