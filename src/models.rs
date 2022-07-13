use rocket_sync_db_pools::diesel::Queryable;
#[derive(Queryable)]
pub struct User {
    id :i32,
    name: String,
    age : u32
}
 
