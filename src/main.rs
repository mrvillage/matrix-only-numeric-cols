use std::{collections::HashSet, path::Path};

fn matrix_is_numeric(reader: impl std::io::Read, output: &Path) {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_reader(reader);
    let headers = reader.headers().unwrap();
    // Check if headers are numeric.
    let colnames = if headers.iter().next().unwrap().parse::<f64>().is_err() {
        Some(headers.iter().map(|x| x.to_string()).collect::<Vec<_>>())
    } else {
        None
    };
    let mut data = Vec::new();
    let mut non_numeric = HashSet::new();
    let _ = headers;
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
    let mut writer = csv::WriterBuilder::new()
        .delimiter(b'\t')
        .from_writer(std::fs::File::create(output).unwrap());
    if let Some(colnames) = colnames {
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
    }
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
    let path = std::env::args().nth(1).expect("no file given");
    let path = Path::new(&path);
    let output = std::env::args().nth(2).expect("no output file given");
    let output = Path::new(&output);
    let file = std::fs::File::open(path).expect("file not found");
    if path.extension().unwrap() == "gz" {
        matrix_is_numeric(flate2::read::GzDecoder::new(file), output)
    } else {
        matrix_is_numeric(file, output)
    };
}
