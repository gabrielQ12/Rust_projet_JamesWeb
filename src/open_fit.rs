use std::fs::File;
use std::io::Read;
use std::str;
use std::collections::btree_map::BTreeMap;

pub struct Hud {
    header: BTreeMap<String, String>,
    data: Vec<f32>,
    
}

pub fn open_fits(path: &str) -> Result<(), std::io::Error> 

{
    let mut file = File::open(path)?;
    
    let _hud1 = read_hud(&mut file);
    let hud2 = read_hud(&mut file);
    
    
    for (key, value) in hud2.header.iter()
    {
        println!("{:?}=> {:?}", key, value);
    }
    
    Ok (())
}



fn read_hud(file: &mut File) -> Hud {
    let hud = Hud {
        header : read_header(file),
        data : Vec::new(),
    };
    
    
    
    
    return hud;
    
}

fn read_header(file: &mut File) -> BTreeMap<String, String> {
    let mut stop_reading = false;
    let mut header: BTreeMap<String, String> = BTreeMap::new();
    
    while stop_reading == false 
    {
        if let Ok(block) =  read_block(file)
        {
            let header_parts = block.chunks(80);
            
            for header_part in header_parts 
            {
                let row = str::from_utf8(header_part).unwrap().trim();
                
                if row == "END" {
                    stop_reading = true; 
                }
                
                else 
                
                {
                    if let Some ((key, value)) = extract_key_value(row)
                    {
                        header.insert(key, value);
                    }
                }
            } 
        }
        else   
        {
            stop_reading = true;
        };
    }
    return header
}    


fn read_block(file: &mut File) -> Result<Vec<u8>, std::io::Error>

{
    let mut buffer = vec![0; 2880];
    file.read_exact(&mut buffer)?;
    
    Ok(buffer)
    
}


fn extract_key_value(row: &str) -> Option<(String, String)> 
{
    if row.find("= ") == Some(8) 
    {
        
        let (key, right_part) = row.split_once("= ")?;
        let mut value = right_part;
        if let Some((v, _comment)) = right_part.split_once("/") 
        {
            
            
            value = v;
        }
        return Some((String::from(key.trim()), String::from(value.trim())))
        
    }
    return None;
}



