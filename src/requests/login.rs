
use rocket_contrib::templates::Template;
use rocket::http::{Cookies, Cookie};
use rocket::request::Form;
use rocket::response::Redirect;
use log::{info};

use crate::db::forms::{LoginForm, RegistrationForm};
use crate::page_context::PageContext;




#[get("/login?<login_error>")]
pub fn login(
    db_connection: crate::LogsDbConn, 
    mut cookies: Cookies,
    login_error: Option<String>) -> Template {

    let mut ctx = PageContext::from(&mut cookies, &db_connection);
    ctx.login_error = login_error;
    Template::render("login", &ctx)
}



#[post("/login", data="<login_form>", rank=1)]
pub fn submit_login(
    db_connection: crate::LogsDbConn, 
    login_form: Form<LoginForm>, 
    mut cookies: Cookies) -> Result<Redirect, Template> {
    
    let ctx = PageContext::from(&mut cookies, &db_connection);
    let login_form = login_form.into_inner();
    match login_form.login(&db_connection) {
        
        Ok(logged_user) => {
            info!(target: "Site info", "Login succeeded '{}'", logged_user.username);
            cookies.add_private(Cookie::new("user_id", logged_user.id.to_string()));
            Ok(Redirect::to("/home"))
        },

        Err(e) => {
            info!(target: "Site info", "Login failed with error: {:?}", e);
            Err(Template::render(
                "login", 
                ctx.with_login_error(e)))
        }
    }
}

#[post("/register", data="<reg_form>", rank=0)]
pub fn submit_registration(
    db_connection: crate::LogsDbConn, 
    reg_form: Form<RegistrationForm>, 
    mut cookies: Cookies
) -> Result<Redirect, Template> {

    info!(target: "Site info", "Registration form submitted: {:?}", reg_form);

    let ctx = PageContext::from(&mut cookies, &db_connection);

    let reg_form = reg_form.into_inner();
    let usr = reg_form.register(&db_connection);
    
    match usr {
        Err(a) => Err(Template::render(
            "login", 
            ctx.with_registration_error(a)
        )),
        Ok(usr) => {
            ctx.with_user(usr).update_cookies(&mut cookies);
            Ok(Redirect::to("/home"))
        }
    }
}
