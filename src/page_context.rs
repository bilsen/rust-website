use rocket::http::{Cookies, Cookie};

use crate::error::*;
use crate::db::user::User;
// Information needed to render templates
#[derive(Serialize)]
pub struct PageContext {

    // The user that's logged in
    pub user: Option<User>,
    
    // Login error returned
    pub login_error: Option<String>,

    // Registration error returned
    pub registration_error: Option<String>,

}

impl PageContext {

    pub fn from(
        cookies: &mut Cookies, 
        db_connection: &crate::LogsDbConn) -> PageContext
    {

        let user = User::from_cookies(db_connection, cookies);


        PageContext {
            user: user,
            login_error: None,
            registration_error: None
        }
    }
    pub fn with_user(self, usr: User) -> Self {
        PageContext {
            user: Some(usr),
            ..self
        }
    }
    pub fn with_login_error(self, err: LoginError) -> Self {
        
        PageContext {
            login_error: Some(err.to_string()),
            ..self
        }
    
    }
    pub fn with_registration_error(self, err: RegistrationError) -> Self {
        PageContext {
            registration_error: Some(err.to_string()),
            ..self
        }
    }
}


impl PageContext {

    pub fn update_cookies(&self, cookies: &mut Cookies) {

        if let Some(usr) = &self.user {
            cookies.add_private(Cookie::new("user_id", usr.id.to_string()));
        }
    }
}