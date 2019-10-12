use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct CSVWriter {
    buf_out: BufWriter<File>,
}

impl CSVWriter {
    pub fn new(file_name: &str) -> CSVWriter {
        CSVWriter {
            buf_out: BufWriter::new(
                File::create(file_name)
                    .expect(format!("Can not create file {}", file_name).as_str()),
            ),
        }
    }
}
