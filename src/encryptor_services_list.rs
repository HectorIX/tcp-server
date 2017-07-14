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
        _ => {
            "A typpo error! Please try again...\n[Tip: type help]\n".to_string()
        },


    }
}
