
use std::fs::File;
use std::io::Read;
use::std::str;

fn main() {
    let mut file = File::open("src/picture/jw02420001001_gs-fg_2022192192555_cal.fits").unwrap();

    let mut buffer = vec![0; 100];
    file.read_exact(&mut buffer);

    println!("{:?}", buffer);
}
