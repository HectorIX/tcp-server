use parser;
use file_io;


struct User {

    id : u64,
    username : String,
    session_key : String,
}

trait whoami {

    fn username(&self) -> String;
    fn session_key(&self) -> String;
    fn id(&self) -> u64;
}

impl whoami for User {

    fn username(&self) -> String {
        //self.username
        "user".to_string()
    }

    fn session_key(&self) -> String {
        //self.session_key
        "key".to_string()
    }

    fn id(&self) -> u64 {
        self.id
    }
}

pub fn sign_in_service(credentials:String) -> String {

    let path = "database/user-credentials.txt";
    let (username,password) = parser::split_credentials(credentials);

    if verify_user(path.to_string(), username, password) {
        format!("\n\n\t=> Welcome User!\n")
    }
    else {
        format!("\n\n\t*** Either your username or password are incorrect.\n
                     \t    Please try again...\n")
    }

}


fn verify_user( path:String, username:String, password:String ) -> bool {

    let file_context:String = file_io::read_file(path);


    //let names: Vec<&str> = file_context.split("username::").collect();


    if file_context.contains(username.as_str()) {

        let legitimate_password = "myPass".to_string();

        match password {

            _ if legitimate_password == password =>  { return true; }
            _ => {return false;}
        }
    }


    false
}
