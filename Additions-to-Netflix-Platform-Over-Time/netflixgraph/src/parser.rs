use csv::ReaderBuilder;
use chrono::{NaiveDate, Datelike};
use std::collections::HashMap;
use std::error::Error;
use serde::Deserialize;

#[derive(Deserialize)]
struct NetflixRecord {
    #[serde(deserialize_with = "deserialize_optional_date")]
    date_added: Option<NaiveDate>,
}

fn deserialize_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => {
            let str = str.trim();
            match NaiveDate::parse_from_str(&str, "%B %d, %Y") {
                Ok(date) => Ok(Some(date)),
                Err(_) => {
                    eprintln!("Warning: Skipping invalid date format: {}", str);
                    Ok(None)
                },
            }
        },
        None => Ok(None),
    }
}

pub fn parse_netflix_data(file_path: &str) -> Result<HashMap<i32, usize>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file_path)?;

    let mut year_count: HashMap<i32, usize> = HashMap::new();
    let mut missing_dates_count = 0;

    for result in rdr.deserialize() {
        let record: NetflixRecord = match result {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Error deserializing record: {}", e);
                continue;
            }
        };

        if let Some(date) = record.date_added {
            *year_count.entry(date.year()).or_insert(0) += 1;
        } else {
            missing_dates_count += 1;
        }
    }

    let mut sorted_years: Vec<_> = year_count.into_iter().collect();
    sorted_years.sort_by_key(|&(year, _)| year);

    let sorted_year_count = sorted_years.into_iter().collect();

    println!("Records with missing dates: {}", missing_dates_count);
    Ok(sorted_year_count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    // temporary csv file
    fn create_temp_csv(contents: &str) -> NamedTempFile {
        let mut file = NamedTempFile::new().unwrap();
        write!(file.as_file_mut(), "{}", contents).unwrap();
        file
    }

    #[test]
    fn test_parse_netflix_data() {
        let data = "\
        \"September 25, 2021\"\n\
        \"September 24, 2021\"\n\
        \"September 23, 2021\"\n\
        \"Invalid Date\"\n";


        let file = create_temp_csv(data);
        let year_count = parse_netflix_data(file.path().to_str().unwrap()).unwrap();

        // is year correctly counted?
        assert_eq!(year_count.get(&2021), Some(&3));

        // are invalid dates skipped?
        assert_eq!(year_count.contains_key(&2020), false);
    }
}
