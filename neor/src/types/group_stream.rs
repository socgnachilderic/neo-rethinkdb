use std::fmt::Debug;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use super::ReqlType;

#[derive(Debug, Default, Clone, Serialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GroupedStream<G: DeserializeOwned + Serialize, V: DeserializeOwned + Serialize>(
    Vec<GroupedItem<G, V>>,
);

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GroupedItem<G, V> {
    pub group: G,
    pub values: Vec<V>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
struct InnerGroup {
    #[serde(rename = "$reql_type$")]
    reql_type: ReqlType,
    data: Vec<[Value; 2]>,
}

impl<'de, G, V> Deserialize<'de> for GroupedStream<G, V>
where
    G: DeserializeOwned + Serialize,
    V: DeserializeOwned + Serialize,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Deserialize::deserialize(deserializer).map(|item| {
            let inner: InnerGroup = item;

            let data: Vec<GroupedItem<G, V>> = inner
                .data
                .into_iter()
                .map(|item| {
                    let group: G = serde_json::from_value(item[0].clone()).unwrap();
                    let values: Vec<V> = serde_json::from_value(item[1].clone()).unwrap();

                    GroupedItem { group, values }
                })
                .collect();

            GroupedStream(data)
        })
    }
}

impl<G, V> GroupedStream<G, V>
where
    G: DeserializeOwned + Serialize,
    V: DeserializeOwned + Serialize,
{
    pub fn collect(self) -> Vec<GroupedItem<G, V>> {
        self.0
    }
}

impl Default for InnerGroup {
    fn default() -> Self {
        Self {
            reql_type: ReqlType::GroupedData,
            data: Default::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use super::GroupedStream;

    #[derive(Serialize, Deserialize, Debug)]
    struct Posts {
        id: u8,
        title: String,
        content: String,
        user_id: u8,
    }

    #[test]
    fn test_group_stream() {
        let data = r#"
        {
            "$reql_type$": "GROUPED_DATA",
            "data": [
                [
                    1,
                    [
                        {
                            "content": "content 5",
                            "id": 5,
                            "title":"title 5",
                            "user_id": 1
                        },
                        {
                            "content": "content 3",
                            "id": 3,
                            "title": "title 3",
                            "user_id": 1
                        },
                        {
                            "content": "content 1",
                            "id": 1,
                            "title": "title 1",
                            "user_id": 1
                        }
                    ]
                ],
                [
                    2,
                    [
                        {
                            "content": "content 4",
                            "id": 4,
                            "title": "title 4",
                            "user_id": 2
                        },
                        {
                            "content": "content 2",
                            "id": 2,
                            "title": "title 2",
                            "user_id": 2
                        }
                    ]
                ]
            ]
        }
        "#;

        let elememt: GroupedStream<u8, Posts> = serde_json::from_str(data).unwrap();
        dbg!(elememt);
    }
}
