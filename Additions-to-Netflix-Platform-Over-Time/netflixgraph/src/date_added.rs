use csv::{ReaderBuilder, WriterBuilder};
use std::error::Error;


pub fn remove_extra_columns(input_file: &str, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().from_path(input_file)?;
    let mut wtr = WriterBuilder::new().from_path(output_file)?;

    for result in rdr.records() {
        let record = result?;
        let date_added = record.get(6).unwrap_or("");

        wtr.write_record(&[date_added])?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_remove_extra_columns() -> Result<(), Box<dyn Error>> {
        // Create a temporary CSV file with test data
        let mut input_file = NamedTempFile::new()?;
        writeln!(input_file, "col1,col2,col3,col4,col5,col6,date_added")?;
        writeln!(input_file, "data1,data2,data3,data4,data5,data6,2021-09-01")?;
        writeln!(input_file, "data1,data2,data3,data4,data5,data6,2021-09-02")?;

        let output_file = NamedTempFile::new()?;

        remove_extra_columns(input_file.path().to_str().unwrap(), output_file.path().to_str().unwrap())?;

        let output_contents = std::fs::read_to_string(output_file.path())?;

        let expected_contents = "2021-09-01\n2021-09-02\n";
        assert_eq!(output_contents, expected_contents);

        Ok(())
    }
}
