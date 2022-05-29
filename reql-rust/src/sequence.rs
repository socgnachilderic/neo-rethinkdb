use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Sequence<T> {
    data: Vec<T>,
    counter: usize,
}

impl<T> Sequence<T> {
    pub fn into_vec(self) -> Vec<T> {
        self.data
    }
}

impl<T: Debug> Debug for Sequence<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Sequence").field(&self.data).finish()
    }
}

impl<T: Clone + Debug> Iterator for Sequence<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.data.get(self.counter) {
            Some(value) => {
                self.counter += 1;
                Some(value.clone())
            }
            None => None,
        }
    }
}

impl<T: Clone + Debug> ExactSizeIterator for Sequence<T> {
    fn len(&self) -> usize {
        self.data.len()
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for Sequence<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer).map(|item| {
            Sequence {
                data: item,
                counter: 0,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::document::Document;

    use super::Sequence;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct User {
        first_name: String,
        last_name: String,
    }

    #[test]
    fn test_sequence() {
        let expected_data = vec![
            User { first_name: "John".to_owned(), last_name: "Doe".to_owned() },
            User { first_name: "Don".to_owned(), last_name: "Juan".to_owned() },
        ];

        let values = serde_json::to_string(&expected_data).unwrap();

        let users: Sequence<Document<User>> = serde_json::from_str(&values).unwrap();
        let users: Vec<User> = users.map(|user| user.get_value())
            .collect();

       assert!(expected_data.eq(&users))
    }
}
