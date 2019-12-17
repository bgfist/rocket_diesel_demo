use rocket::get;

#[get("/list")]
pub fn list() -> &'static str {
    "Hello, world!"
}