use std::fmt;

use serde::{
    de::{self, Visitor}, Deserializer, Serializer
};
use surrealdb::sql::Thing;

/// Deserialize a String into a surrealdb::sql::Thing
pub fn deserialize<'de, D>(deserializer: D) -> Result<Thing, D::Error>
where
    D: Deserializer<'de>,
{

    struct StringVisitor;

    impl<'de> Visitor<'de> for StringVisitor {
        type Value = Thing;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string like 'test:xyz'")
        }

        fn visit_str<E>(self, value: &str) -> Result<Thing, E>
        where
            E: de::Error,
        {
            let splits: Vec<&str> = value.split(':').collect();
            if splits.len() == 2 {
                Ok(Thing::from((splits[0], splits[1])))
            } else {
                Err(de::Error::custom("invalid string"))
            }
        }

        fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            self.visit_str(&v)
        }
    }
    deserializer.deserialize_str(StringVisitor)
}

/// Serialize a surrealdb::sql::Thing into a String
pub fn serialize<S>(thing: &Thing, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&thing.to_raw())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use serde_json;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct StringThing {
        #[serde(with = "super")]
        id: Thing,
    }

    #[test]
    fn test_deserialize() {
        let value = serde_json::json!({ "id": "test:xyz" });
        let result = serde_json::from_value::<StringThing>(value);
        dbg!(&result);
        assert!(result.is_ok());
        if let Ok(s) = result {
            assert_eq!(
                s,
                StringThing {
                    id: Thing::from(("test", "xyz"))
                }
            );
        }
    }

    #[test]
    fn test_serialize_alphanumeric() {
        let test = StringThing {
            id: Thing::from(("test", "xyz")),
        };
        let result = serde_json::to_string(&test);
        dbg!(&result);
        assert!(result.is_ok());
        if let Ok(s) = result {
            assert_eq!(s, r#"{"id":"test:xyz"}"#);
        }
    }

    #[test]
    fn test_serialize_numeric() {
        let test = StringThing {
            id: Thing::from(("test", "123")),
        };
        let result = serde_json::to_string(&test);
        dbg!(&result);
        assert!(result.is_ok());
    }
}
