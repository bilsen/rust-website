// The interface with the site database, powered by diesel

use super::{
    user::{User, InsertableUser},
    forms::{LoginForm, RegistrationForm}
};
use diesel::prelude::*;
use crate::error::*;
use rocket::http::{Cookies};



impl User {

    pub fn from_cookies(db_connection: &crate::LogsDbConn, cookies: &mut Cookies) -> Option<User>
    {
        match cookies.get_private("user_id") {
            Some(cookie) => {
                let user_id = cookie.value().parse::<i32>().expect("Invalid cookies");
                if let Ok(usr) = User::from_id(db_connection, user_id) {
                    Some(usr)
                } else {
                    None
                }
            },
            None => None,
        }
    }

    pub fn from_id(db_connection: &crate::LogsDbConn, user_id: i32) -> Result<User, diesel::result::Error> {
        use crate::schema::users::dsl::*;
        
        users.find(user_id).first(&db_connection.0)
    }
}

impl Problem {
    
}

impl LoginForm {
    
    pub fn login(&self, db_connection: &crate::LogsDbConn) -> Result<User, LoginError> {
        use crate::schema::users::dsl::*;
        
        match users.filter(username.eq(&self.username))
            .filter(password_hash.eq(&self.password))
            .first(&db_connection.0)
        {
            Err(diesel::result::Error::NotFound) => Err(LoginError::WrongPassword),
            Err(a) => Err(LoginError::DieselError(a)),
            Ok(u) => Ok(u)
        }
    }
}

impl RegistrationForm {

    fn get_user(&self) -> InsertableUser {
        use std::fs::File;
        use std::io::BufReader;

        InsertableUser {
            username: self.username.clone(),
            email_adress: self.email_adress.clone(),
            password_hash: self.password.clone(),
            rating: 1000,
            preferences: serde_json::from_reader(
                BufReader::new(
                    File::open("resources/default_preferences.json").expect("Error opening default preferences file"))).expect("Error parsing default preferences json")
        }
    }

    // Attempt to register a new user using form
    pub fn register(&self, db_connection: &crate::LogsDbConn) -> Result<User, RegistrationError> {
        // TODO: hash user passwords
        use crate::schema::users::dsl::*;
        
        // Is form data valid?
        
        
        // Is username unique?
        match users.filter(username.eq(&self.username))
        .count()
        .get_result(&db_connection.0) {
            Ok(0) => Ok(()),
            Ok(_) => Err(RegistrationError::UsernameTaken),
            Err(e) =>  Err(RegistrationError::DieselError(e))
        }?;
        
        // Is email-adress unique?
        match users.filter(email_adress.eq(&self.email_adress))
        .count()
        .get_result(&db_connection.0) {
            Ok(0) => Ok(()),
            Ok(_) => Err(RegistrationError::EmailTaken),
            Err(e) =>  Err(RegistrationError::DieselError(e))
        }?;
        
        
        
        
        
        diesel::insert_into(users)
        .values(&self.get_user())
        .get_result(&db_connection.0)
        .map_err(|e| RegistrationError::DieselError(e))
    }
}


