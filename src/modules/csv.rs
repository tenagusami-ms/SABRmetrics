use std::fs::File;
use csv::StringRecord;
use std::error::Error;
use std::path::Path;

pub fn read_csv(path: &Path) -> Result<Vec<StringRecord>, Box<dyn Error>> {
    let file = File::open(path)?;
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(file);
    Ok(rdr.records().map(|r| r.unwrap()).collect::<Vec<_>>())
}
