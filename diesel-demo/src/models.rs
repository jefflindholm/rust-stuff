use super::schema::posts;

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub author: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub author: &'a str,
    pub title: &'a str,
    pub body: &'a str,
}
