use file_io;
use parser;
use user;


pub fn download_service( data: String ) -> String {


    let mut path = "private_data/".to_string();


    if data.len() > 0 {

        let (username, rest_data) = parser::split_credentials(data);
        let (session_key, filename) = parser::split_session_key(rest_data);


        path.push_str(&username);
        path.push_str("/");
        path.push_str(&filename);

        if (username == user::get_username()) &
           (session_key == user::get_session_key()) {

            let file_context = file_io::read_file(path.to_string());

            let mut file_request = "download_state::OK**".to_string();


            file_request.push_str(&user::get_session_key());
            file_request.push_str("#!?#");
            file_request.push_str(&filename);
            file_request.push_str("<$$>");
            file_request.push_str(&file_context);

            file_request
        }
        else {

            "download_state::SESSION_Expired**".to_string()
        }
    }
    else {

        "download_state::Failed**".to_string()
    }

}
