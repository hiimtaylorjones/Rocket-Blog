#[derive(Debug, PartialEq, Eq, Serialize, Queryable)]
pub struct Post {
    id: i64,
    title: String,
    body: String,
}

#[derive(Debug, Insertable, FromForm, AsChangeset)]
#[table_name="posts"]
pub struct PostForm {
    title: String,
    body: String,
}

impl Post {
    pub fn new(id: i64, title: &str, body: &str) -> Self {
        Post {
            id: id,
            title: title.into(),
            body: body.into(),
        }
    }
}
