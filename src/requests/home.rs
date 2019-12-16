
use rocket::http::Cookies;
use crate::page_context::PageContext;
use rocket_contrib::templates::Template;

#[get("/home")]
pub fn home(mut cookies: Cookies, db_connection: crate::LogsDbConn) -> Template {
    
    let ctx = PageContext::from(&mut cookies, &db_connection);
    
    Template::render("home", &ctx)

}