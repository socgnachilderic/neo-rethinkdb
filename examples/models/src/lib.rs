use futures::stream::{select_all, TryStreamExt};
use neor::r;

pub use neor::Session;

pub mod post;

pub async fn init_db() -> Session {
    let session = r.connection().connect().await.unwrap();
    create_table_if_not_exist(session.clone()).await;

    session
}

async fn create_table_if_not_exist(conn: Session) {
    let tables = [post::Post::tablename()];
    let mut streams = Vec::new();

    for tablename in tables {
        streams.push(r.table_create(tablename).build_query(&conn));
    }

    let mut list = select_all(streams);

    while let Some(resp) = list.try_next().await.unwrap_or_default() {
        dbg!(resp);
    }
}
