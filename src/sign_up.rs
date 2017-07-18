use std::fs;

use file_io;
use parser;


pub fn sign_up_service(data:String) -> String {


    let mut user_folder = "private_data/".to_string();


    if data.len() > 0 {

        let path = "database/user-credentials.txt";
        let (username, password) = parser::split_credentials(data);
        let mut credentials = "<**>\n".to_owned();


        if !dublicate_username(path.to_string(), username.clone()) {

            credentials.push_str("username::");
            credentials.push_str(&username);
            credentials.push_str("\n");
            credentials.push_str("password::");
            credentials.push_str(&password);
            credentials.push_str("\n");
            credentials.push_str("ID::");
            credentials.push_str(&next_id(path.to_string()));
            credentials.push_str("\n");


            user_folder.push_str(&username);

            match fs::create_dir(user_folder) {

                Err(e) => println!("Failed to create folder: {:?}", e.kind()),
                Ok(_)  => {},
            }


            let cred = file_io::write_file(path.to_string(), credentials.to_owned());


            if !cred.starts_with("**Failed") {

                format!("sign_up_state::OK**")
            }
            else {

                format!("sign_up_state::Failure**")
            }

        }
        else {

            format!("sign_up_state::Dublicate**")
        }

    }
    else {

        format!("sign_up_state::Unauthorised**")
    }

}



fn dublicate_username(path:String, username:String) -> bool {


    let file_context = file_io::read_file(path);



    if !file_context.starts_with("**Failed") {

        let vector_of_users: Vec<&str> = file_context.split("<**>\n")
                                                     .collect();

        for user_data in vector_of_users {

            if user_data.to_string().contains(username.as_str()) {

                return true;
            }
        }

        false

    }
    else {

        false
    }


}



fn next_id(path:String) -> String {

    let file_context = file_io::read_file(path);

    let file_vector: Vec<&str> = file_context.split("ID::").collect();
    let last_id = file_vector.last().unwrap();

    let id_vector:Vec<&str> = last_id.split("\n").collect();
    let next_id = id_vector[0].to_string();


    (next_id.to_string().parse::<u64>().unwrap() + 1).to_string()

}
