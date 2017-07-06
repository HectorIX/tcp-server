

pub fn mini_menu() -> String {


    let horizontal_line = "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n";
    let header  =  "\t Welcome to the Rust Encryptor!\n\n";
    let sign_up =  "\t + To sign-up type: Sign-up\n";
    let log_in  = "\t + To login type: Login\n";


    format!{ "{} {} {} {} {}",
                              horizontal_line,
                              header,
                              sign_up,
                              log_in,
                              horizontal_line
    }

}
