use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(PartialEq, Clone)]
pub enum DataType {
    Any,
    Text,
    Number,
    Boolean,
    Date,
    Time,
    DateTime,
    Undefined,
    Null,
}

impl DataType {
    pub fn literal(&self) -> &'static str {
        return match self {
            DataType::Any => "Any",
            DataType::Text => "Text",
            DataType::Number => "Number",
            DataType::Boolean => "Boolean",
            DataType::Date => "Date",
            DataType::Time => "Time",
            DataType::DateTime => "DateTime",
            DataType::Undefined => "Undefined",
            DataType::Null => "Null",
        };
    }
}

lazy_static! {
    pub static ref TABLES_FIELDS_TYPES: HashMap<&'static str, DataType> = {
        let mut map = HashMap::new();
        map.insert("commit_id", DataType::Text);
        map.insert("title", DataType::Text);
        map.insert("message", DataType::Text);
        map.insert("name", DataType::Text);
        map.insert("full_name", DataType::Text);
        map.insert("insertions", DataType::Number);
        map.insert("deletions", DataType::Number);
        map.insert("files_changed", DataType::Number);
        map.insert("email", DataType::Text);
        map.insert("type", DataType::Text);
        map.insert("time", DataType::DateTime);
        map.insert("is_head", DataType::Boolean);
        map.insert("is_remote", DataType::Boolean);
        map.insert("commit_count", DataType::Number);
        map.insert("repo", DataType::Text);
        map
    };
}
