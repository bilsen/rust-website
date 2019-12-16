
#[derive(FromForm, Debug)]
pub struct LoginForm {
    pub username: String,
    pub password: String
}



#[derive(FromForm, Debug)]
pub struct RegistrationForm {
    pub username: String,
    pub password: String,
    pub email_adress: String,
}

