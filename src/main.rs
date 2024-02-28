use std::fs::File;
use std::io::Read;
use std::str;

fn main() -> Result<(), std::io::Error> {
    let mut file = File::open("src/picture/jw02420001001_gs-fg_2022192192555_cal.fits")?;
    
    let mut buffer = vec![0; 2880];
    file.read_exact(&mut buffer)?;
    
    let header_parts: Vec<&[u8]> = buffer.chunks(80).collect();
    
    for header_part in header_parts {
        println!("> {:?}", str::from_utf8(header_part).unwrap());
    }
    
    Ok(())
}