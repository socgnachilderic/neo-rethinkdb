use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Sequence<T> {
    data: Vec<T>,
    lenght: usize,
}

impl<T> Sequence<T> {
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Clone> Iterator for Sequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.get(self.lenght) {
            Some(value) => {
                self.lenght += 1;
                Some(value.clone())
            }
            None => None,
        }
    }
}

impl<T: Clone> ExactSizeIterator for Sequence<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Sequence<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer).map(|item| {
            let mut seq = Sequence {
                data: item,
                lenght: 0,
            };

            seq.lenght = seq.data.len();

            seq
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::document::Document;

    use super::Sequence;

    #[derive(Debug, Serialize, Deserialize)]
    struct User {
        first_name: String,
        last_name: String,
    }

    #[test]
    fn test_sequence() {
        let value = r#"
            [
                { "first_name": "John", "last_name": "Doe" },
                { "first_name": "Don", "last_name": "Juan" }
            ]
        "#;

        let users: Sequence<Document<User>> = serde_json::from_str(&value).unwrap();

        dbg!(users);
    }
}
