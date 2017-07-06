use encryptor_services_list;



pub fn state( client_request:String ) -> String {

    let state_tupple = request_splitter(client_request);

    let the_state = state_tupple.0;
    let the_request = state_tupple.1;


    match the_state.as_ref() {

        "informatic_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "sign_up_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "upload_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "download_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "integrity_state" => {
            encryptor_services_list::service_list(the_request)
        }
        _ => {
            "Undefined State...".to_string()
        },

    }
}

fn request_splitter( client_request:String ) -> (String,String) {

    let req_vector: Vec<&str> = client_request.split("::").collect();
    let (the_state,request) = (req_vector[0].to_string(), req_vector[1].to_string());

    (the_state, request)
}
