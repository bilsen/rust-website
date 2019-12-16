
#[derive(Debug)]
pub enum LoginError {
    WrongPassword,
    DieselError(diesel::result::Error)
}

#[derive(Debug)]
pub enum RegistrationError {
    UsernameTaken,
    EmailTaken,
    DieselError(diesel::result::Error)
}


impl ToString for LoginError {
    fn to_string(&self) -> String {
        match self {
            LoginError::WrongPassword => format!("Invalid password"),
            LoginError::DieselError(e) => format!("Internal server error: {}", e)
        }
    }
}

impl ToString for RegistrationError {
    fn to_string(&self) -> String {
        match self {
            RegistrationError::UsernameTaken => format!("Username is already in use"),
            RegistrationError::EmailTaken => format!("Email-adress is already in use"),
            RegistrationError::DieselError(e) => format!("Internal server error: {}", e)
        }
    }
}