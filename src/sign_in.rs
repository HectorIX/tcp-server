extern crate crypto;
extern crate rand;

use parser;
use file_io;
use user;

use std::str;

use self::rand::{ thread_rng, Rng };
use self::crypto::whirlpool::Whirlpool;
use self::crypto::digest::Digest;



pub fn sign_in_service(credentials:String) -> String {


    if credentials.len() > 0 {

        let path = "database/user-credentials.txt";
        let (username,password) = parser::split_credentials(credentials);



        if verify_user(path.to_string(), username.clone(), password) {

            user::set_username(username);
            user::set_session_key(session_key_maker());
            user::set_user_status(true);


            format!("sign_in_state::OK**{}--{}", user::get_session_key()
                                               , user::get_username())

        }
        else {

            format!("sign_in_state::NOT_Mactching**")

        }
    }
    else {

        format!("sign_in_state::ALREADY_Sign_in**")
    }

}


fn verify_user( path:String, username:String, password:String ) -> bool {

    let file_context:String = file_io::read_file(path);
    let vector_of_users: Vec<&str> = file_context.split("<**>\n").collect();


    for user_data in vector_of_users {


        if user_data.to_string().contains(username.as_str()) {

            let legitimate_password = return_pass(user_data.to_string());

            match password {

                _ if legitimate_password == password =>  { return true; }
                _ => {return false;}
            }
        }
    }


    false
}


fn return_pass(user_data:String) -> String {

    let v:Vec<&str> = user_data.split("password::").collect();
    let v_plus = v[1].to_string();
    let v_minus:Vec<&str> = v_plus.split("\nID::").collect();

    let password:String = v_minus[0].to_string();

    password

}


fn session_key_maker() -> String {

    let mut whirlpool_hasher = Whirlpool::new();
    let random_key: String = thread_rng().gen_ascii_chars()
                                         .take(128)
                                         .collect();


    whirlpool_hasher.input_str(&random_key);
    let session_key = whirlpool_hasher.result_str();

    session_key

}
