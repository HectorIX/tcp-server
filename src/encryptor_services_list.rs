use service_state;
use main_menu;
use mini_menu;
use sign_up;



pub fn service_list( request_service:String ) -> String {

    let service_and_data = data_splitter(request_service);

    let service = service_and_data.0;
    let data = service_and_data.1;


    match service.as_ref() {

        "start" => {
            mini_menu::mini_menu()
        },
        "menu" => {
            main_menu::main_menu()
        },
        "info" => {
            "About Encryptor doc".to_string()
        },
        "help" => {
            "Welcome new user!!!\n
            \t+ To start using our serveices type: start\n
            \t+ To get more info about our services type: info\n".to_string()
        },
        "Sign-up" => {
            sign_up::sign_up_service(data)
        },
        "Sign-in" => {
            "sign-in process".to_string()
        },
        "Download" => {
            "download process".to_string()
        },
        "Upload" => {
            "upload process".to_string()
        },
        "Integrity" => {
            "integrity verification".to_string()
        },
        _ => {
            "A typpo error! Please try again...\n[Tip: type help]\n".to_string()
        },


    }
}

fn data_splitter( request_service:String ) -> (String,String) {

    let req_vector: Vec<&str> = request_service.split("**").collect();
    let (service,data) = (req_vector[0].to_string(), req_vector[1].to_string());

    (service, data)
}
