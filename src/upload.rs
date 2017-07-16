use file_io;
use parser;
use user;



pub fn upload_service(data:String) -> String {

    let mut path = "private_data/".to_string();


    if data.len() > 0 {

        let (username, rest_data) = parser::split_credentials(data);
        let (session_key, file_data) = parser::split_session_key(rest_data.clone());
        let (filename, file_context) = parser::split_filename_from_context(file_data.clone());


        path.push_str(&username);
        path.push_str("/");
        path.push_str(&filename);

        if (username == user::get_username()) &
           (session_key == user::get_session_key()) {

            file_io::write_file(path, file_context);
            "upload_state::OK**".to_string()
         }
         else {

             "upload_state::SESSION_Expired**".to_string()
         }


    }
    else {

        "upload_state::Failed**".to_string()
    }

}
