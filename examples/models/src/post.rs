use neor::arguments::{InsertOption, ReturnChanges};
use neor::types::MutationResponse;
use neor::{args, r, Command, Converter, Result, Session};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub async fn find_all(conn: &Session) -> Vec<Post> {
        parse_data::<Vec<Post>>(Self::table().run(conn).await)
    }

    pub async fn find_one(conn: &Session, id: &str) -> Option<Post> {
        parse_data::<Option<Post>>(Self::table().get(id).run(conn).await)
    }

    pub async fn create_post(conn: &Session, title: &str, text: &str) -> Post {
        let insert_option = InsertOption::default().return_changes(ReturnChanges::Bool(true));
        let resp = parse_data::<MutationResponse>(
            Self::table()
                .insert(args!(
                    json!({
                        "title": title,
                        "text": text
                    }),
                    insert_option
                ))
                .run(conn)
                .await,
        )
        .changes
        .unwrap();

        let resp = resp
            .first()
            .unwrap()
            .to_owned()
            .new_val
            .unwrap()
            .parse::<Post>()
            .unwrap();

        resp
    }

    pub fn table() -> Command {
        r.table(Self::tablename())
    }

    pub fn tablename() -> &'static str {
        "posts"
    }
}

fn parse_data<T>(value: Result<Option<Value>>) -> T
where
    T: Serialize + DeserializeOwned + Unpin,
{
    value.unwrap().unwrap().parse::<T>().unwrap()
}
