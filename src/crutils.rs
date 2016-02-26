use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};

pub fn write_values<T: AsRef<Path>>(vec: &Vec<f32>, path: T) {
    let file = File::create(path).unwrap();
    let mut writer = BufWriter::new(file);

    for el in vec {
        writer.write_f32::<LittleEndian>(*el).unwrap();
    }
}

pub fn read_values<T: AsRef<Path>>(path: T) -> Vec<f32> {
    let mut file = File::open(path).unwrap();
    let mut vec = Vec::new();

    loop {
        match file.read_f32::<LittleEndian>() {
            Ok(x) => vec.push(x),
            Err(_) => break,
        };
    }
    vec
}
