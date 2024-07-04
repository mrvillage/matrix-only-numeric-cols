use std::{collections::HashSet, path::Path};

fn matrix_is_numeric(files: Vec<String>, output: &Path) {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(std::fs::File::open(&files[0]).unwrap());
    let headers = reader.headers().unwrap();
    // Check if headers are numeric.
    let colnames = headers.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    let mut data = Vec::new();
    let mut non_numeric = HashSet::new();
    let _ = headers;
    for file in files {
        let mut reader = csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(std::fs::File::open(&file).unwrap());
        let headers = reader.headers().unwrap();
        let c = headers.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        if c != colnames {
            panic!("Column names do not match");
        }
        for result in reader.records() {
            let record = result.unwrap();
            for (i, field) in record.iter().enumerate() {
                if field == "NA" {
                    continue;
                }
                if field.parse::<f64>().is_err() {
                    non_numeric.insert(i);
                }
            }
            data.push(record);
        }
    }
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b',')
        .from_writer(std::fs::File::create(output).unwrap());
    let colnames = colnames
        .iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if non_numeric.contains(&i) {
                None
            } else {
                Some(x)
            }
        })
        .collect::<Vec<_>>();
    writer.write_record(&colnames).unwrap();
    for record in data {
        let record = record
            .iter()
            .enumerate()
            .filter_map(|(i, x)| {
                if non_numeric.contains(&i) {
                    None
                } else {
                    Some(x)
                }
            })
            .collect::<Vec<_>>();
        writer.write_record(&record).unwrap();
    }
}

fn main() {
    let output = std::env::args().nth(1).expect("no output file given");
    let output = Path::new(&output);
    let files = std::env::args().skip(2).collect::<Vec<_>>();
    matrix_is_numeric(files, output)
}
