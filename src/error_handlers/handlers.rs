use rocket::Request;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Hey!! looks like you Lost, url {} doesn't exists", req.uri())
}