use auth_service::{Credentials};
fn main() {
    let creds = Credentials {
    username: "let'sgetrusty".to_owned(),
    password:"password123".to_owned()
    };
    auth_service::authenticate(creds);
}//two ways 1. go to model.rs and change username and password to public 
// 2. keep the fields private and create an implementation block with an association called new