use encryptor_services_list;
use parser;


pub fn state( client_request:String ) -> String {

    let state_tupple = parser::request_splitter(client_request);

    let the_state = state_tupple.0;
    let the_request = state_tupple.1;

    println!("state = {:?}", the_state );
    println!("the_request = {:?}", the_request);

    match the_state.as_ref() {

        "informatic_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "sign_up_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "sign_in_state" => {
            encryptor_services_list::service_list(the_request)
        }
        "upload_state" => {
            encryptor_services_list::service_list(the_request)
        },
        "download_state" => {
            encryptor_services_list::service_list(the_request)
        },
        _ => {
            "Undefined State...".to_string()
        },

    }
}
