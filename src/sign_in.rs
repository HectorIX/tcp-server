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


    let vector_of_users: Vec<&str> = file_context.split("<**>").collect();

    for user_data in vector_of_users {

        if user_data.to_string().contains(username.as_str()) {

            let legitimate_password = return_pass(user_data.to_string());

            match password {

                _ if legitimate_password == password =>  { return true; }
                _ => {return false;}
            }
        }
    }

    /*
    if file_context.contains(username.as_str()) {

        let legitimate_password = "myPass".to_string();

        match password {

            _ if legitimate_password == password =>  { return true; }
            _ => {return false;}
        }
    }
    */

    false
}


fn return_pass(user_data:String) -> String {

    let v:Vec<&str> = user_data.split("password::").collect();
    let v_plus = v[1].to_string();
    let v_minus:Vec<&str> = v_plus.split("\nID::").collect();

    let password:String = v_minus[0].to_string();

    password

}
