use std::fs::File;
use std::io::Read;
use std::str;
use std::collections::btree_map::BTreeMap;



pub fn open_fits(path: &str) -> Result<(), std::io::Error> 

{
    let mut file = File::open(path)?;
    
    read_hud(&mut file);
    
    Ok (())
}



fn read_hud(file: &mut File)
{
    let header = read_header(file);
    
    println!("{:?}", header.len());
    
    for (key, value) in header.iter()
    {
        println!("{}: {}", key, value);
    }
}

fn read_header(file: &mut File) -> BTreeMap<String, String> {
    let mut stop_reading = false;
    let mut header: BTreeMap<String, String> = BTreeMap::new();
    
    while stop_reading == false {
        println!("New block");
        
        let block = match read_block(file) {
            Ok(block) => block,
            Err(_error) => return header,
        };
        
        let header_parts = block.chunks(80);
        
        for header_part in header_parts {
            let row = str::from_utf8(header_part).unwrap().trim();
            
            if row == "END" {
                stop_reading = true;
                break;
            }
            
            
            if row.find("= ") == Some(8) 
            {
                
                if let Some((key, right_part)) = row.split_once("= ") {
                    let mut value = right_part.trim();
                    if let Some((v, _comment)) = right_part.split_once("/") 
                    {
                        
                        
                        value = v.trim();
                    }
                    
                    header.insert(key.trim().to_string(), value.trim().to_string());
                } else {
                    continue;
                }
            } else {
                continue;
            }
            
            let parts = row.split_once("=");
            
            if parts.is_none() {
                continue;
            }
        }
    }
    
    header
}



fn read_block(file: &mut File) -> Result<Vec<u8>, std::io::Error>

{
    let mut buffer = vec![0; 2880];
    file.read_exact(&mut buffer)?;
    
    Ok(buffer)
    
}
