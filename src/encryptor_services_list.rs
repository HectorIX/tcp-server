use main_menu;
use mini_menu;
use sign_up;
use sign_in;
use upload;
use parser;



pub fn service_list( request_service:String ) -> String {

    let service_and_data = parser::data_splitter(request_service);

    let service = service_and_data.0;
    let data = service_and_data.1;

    println!("service = {:?}", service );
    println!("data = {:?}", data  );
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
            "\n\t   ================   HELP MENU   ===================\n
            <+> To use our clinet side services type: local
            <+> To use our server side services type: net\n".to_string()
        },
        "Sign-up" => {
            sign_up::sign_up_service(data)
        },
        "Sign-in" => {
            sign_in::sign_in_service(data)
        },
        "Upload" => {
            upload::upload_service(data)
        },
        "Download" => {
            "download process".to_string()
        },
        "Integrity" => {
            "integrity verification".to_string()
        },
        _ => {
            "A typpo error! Please try again...\n[Tip: type help]\n".to_string()
        },


    }
}
