

pub fn main_menu() -> String {


    let horizontal_line = "==============================================================\n";
    let header    = "\t\t      SERVER MENU\n\n";
    let exit      = "\t <:> To exit type: Exit\n";
    let sign_in   = "\t <:> To sign-in type: Sign-in\n";
    let sign_up   = "\t <:> To sign-up type: Sign-up\n";
    let upload    = "\t <:> To upload an encryped file type: Upload\n";
    let download  = "\t <:> To download a file of yours type: Download\n";
    let integrity = "\t <:> To verify the integrity of your files type: Integrity\n\n";




    format!{ "{} {} {} {} {} {} {} {} {}",
                                    horizontal_line,
                                    header,
                                    exit,
                                    sign_up,
                                    sign_in,

    format!{ "{} {} {} {} {} {} {}",
                                    horizontal_line,
                                    header,
                                    exit,
                                    sign_up,
                                    sign_in,
                                    upload,
                                    download,
                                    integrity,
                                    horizontal_line
    }

}
