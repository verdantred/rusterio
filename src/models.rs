use schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Insertable, Associations, AsChangeset)]
#[table_name="users"]
pub struct User{
    pub id: i32,
    pub name: String,
    pub email: String,
    pub passhash: String,
}
