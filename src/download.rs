use file_io;
use parser;


pub fn download_service( data: String ) -> String {


    let mut path = "private_data/".to_string();


    if data.len() > 0 {

        let (username, filename) = parser::split_credentials(data);


        path.push_str(&username);
        path.push_str("/");
        path.push_str(&filename);

        let file_context = file_io::read_file(path.to_string());

        let mut file_request = "download_state::OK**".to_string();

        file_request.push_str(&filename);
        file_request.push_str("<$$>");
        file_request.push_str(&file_context);

        file_request
    }
    else {

        "download_state::Failed**".to_string()
    }

}
