use std;
use serde_json::Value;

pub(crate) fn read_database(path: std::path::PathBuf) -> Value {
    if let Ok(file_content) = std::fs::read_to_string(path) {
        return serde_json::from_str::<Value>(&file_content)
            .expect("File is not JSON");
        //todo: add a struct for for the db file
    } else {
        panic!("File not found")
    }
}