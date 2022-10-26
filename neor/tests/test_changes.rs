use futures::TryStreamExt;
use neor::arguments::ChangesOption;
use neor::types::ChangesResponse;
use neor::{Converter, Result};

use common::{set_up, tear_down, Post};

mod common;

#[tokio::test]
async fn test_limit_data() -> Result<()> {
    let data = Post::get_many_data();
    let (session, table, table_name) = set_up(true).await?;
    let mut response: Vec<Post> = Vec::new();
    let mut connection = session.connection()?;
    let conn = connection.clone();
    let changes_options = ChangesOption::default()
        .include_initial(true)
        .include_states(true)
        .include_types(true);

    let mut query = table.changes(changes_options).build_query(conn);

    while let Some(value) = query.try_next().await? {
        let alt_response = value.parse::<Vec<ChangesResponse<Post>>>()?;

        if alt_response.len() > 0 {
            response = alt_response
                .into_iter()
                .filter(|resp| resp.state.is_none())
                .map(|resp| resp.new_val.unwrap())
                .collect::<Vec<Post>>();

            response.sort_by_key(|post| post.id);
        }

        connection.close(false).await?;
        break;
    }

    assert_eq!(response, data);

    tear_down(session, &table_name).await
}
