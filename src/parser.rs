

pub fn parser( instruction:String ) -> bool {



    if instruction.starts_with("informatic_state") |
       instruction.starts_with("sign_up_state")    |
       instruction.starts_with("sign_in_state")    |
       instruction.starts_with("upload_state")     |
       instruction.starts_with("download_state")   |
       instruction.starts_with("integrity_state")  {

           if instruction.contains("::") {
               return true;
           }
           else {
               return false;
           }

    }

    return false;

}


pub fn request_splitter( client_request:String ) -> (String, String) {

    let req_vector: Vec<&str> = client_request.split("::").collect();
    let (the_state,request) = (req_vector[0].to_string(), req_vector[1].to_string());

    (the_state, request)
}


pub fn data_splitter( request_service:String ) -> (String, String) {

    let req_vector: Vec<&str> = request_service.split("**").collect();
    let (service,data) = (req_vector[0].to_string(), req_vector[1].to_string());

    (service, data)
}


pub fn split_credentials(data:String) -> (String, String)  {

    let data_vector: Vec<&str> = data.split("--").collect();
    let (username,password) = (data_vector[0].to_owned(), data_vector[1].to_owned());

    (username, password)
}


pub fn split_filename_from_context(data:String) -> (String, String)  {

    let data_vector: Vec<&str> = data.split("<$$>").collect();
    let (filename, file_context) = (data_vector[0].to_owned(), data_vector[1].to_owned());

    (filename, file_context)
}


pub fn split_session_key(data:String) -> (String, String)  {

    let data_vector: Vec<&str> = data.split("#!?#").collect();
    let (session_key, file_data) = (data_vector[0].to_owned(), data_vector[1].to_owned());

    (session_key, file_data)
}
