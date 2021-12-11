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


pub fn takeColumns(csvRecords: Vec<Vec<String>>, indices: Vec<usize>) -> Vec<Vec<String>> {
    csvRecords.iter()
        .map(|records|
            indices.iter()
                .map(|&index| records.get(index).unwrap().to_string())
                .collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
