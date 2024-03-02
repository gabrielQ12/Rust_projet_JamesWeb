

mod open_fit;




fn main() -> Result<(), std::io::Error> {

    open_fit::open_fits("src/picture/jw02420001001_gs-fg_2022192192555_cal.fits")?;
    
    Ok(())
}
    
