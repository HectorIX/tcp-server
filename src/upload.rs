use file_io;
use parser;



pub fn upload_service(data:String) -> String {

    let mut path = "private_data/".to_string();


    if data.len() > 0 {

        let (username, rest_data) = parser::split_credentials(data);
        let (filename, file_context) = parser::split_filename_from_context(rest_data.clone());


        path.push_str(&username);
        path.push_str("/");
        path.push_str(&filename);


        file_io::write_file(path, file_context);

        "upload_state::OK**".to_string()

    }
    else {

        "upload_state::Failed**".to_string()
    }

}
