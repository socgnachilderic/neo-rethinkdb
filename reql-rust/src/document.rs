use serde::{de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Document<T>(pub(crate) T);

impl<T> Document<T> {
    pub fn get_value(self) -> T {
        self.0
    }
}

impl<T: Serialize> Serialize for Document<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Document<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer).map(|item| Document(item))
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::Document;

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        first_name: String,
        last_name: String,
    }

    #[test]
    fn test_document() {
        let value = r#"
            {
                "first_name": "John", 
                "last_name": "Doe"
            }
        "#;

        let documents: Document<User> = serde_json::from_str(&value).unwrap();

        dbg!(documents);
    }
}
