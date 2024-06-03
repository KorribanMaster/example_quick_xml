/*
[dependencies]
serde = { version = "1.0", features = ["derive"] }
quick-xml = {version="0.28", features = ["serialize"]}
*/
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Sources {
    #[serde(rename = "Source")]
    source: Vec<Source>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Source {
    A,
    B,
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use quick_xml::de::from_str;
    use quick_xml::se::to_string;

    use super::*;
    #[test]
    fn test_serialize_sources() {
        let sources = Sources {
            source: vec![Source::A, Source::B],
        };
        let xml = r#"<Sources><Source>A</Source><Source>B</Source></Sources>"#;
        let serialized = to_string(&sources).expect("Failed to serialize sources"); // Use the to_string function from quick_xml
        assert_eq!(serialized, xml);
    }
    #[test]
    fn test_deserialize_sources() {
        let xml = r#"<Sources><Source>A</Source><Source>B</Source></Sources>"#;
        let sources = Sources {
            source: vec![Source::A, Source::B],
        };
        let deserialize = from_str::<Sources>(&xml).expect("Failed to deserialize sources");
        assert_eq!(deserialize, sources);
    }
    #[test]
    fn test_deserialize_source() {
        let xml = r#"<Sources><Source>B</Source></Sources>"#;
        let sources = Sources {
            source: vec![Source::B],
        };
        let deserialize = from_str::<Sources>(&xml).expect("Failed to deserialize sources");
        assert_eq!(deserialize, sources);
    }
}
