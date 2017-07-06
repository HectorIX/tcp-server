

pub fn main_menu() -> String {

    let horizontal_line = "%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%\n";
    let header    = "\t\t +++  MAIN MENU   +++\n\n";
    let exit      = "\t <:> To exit type: Exit\n";
    let upload    = "\t <:> To upload an encryped file type: Upload\n";
    let download  = "\t <:> To download a file of yours type: Download\n";
    let integrity = "\t <:> To verify the integrity of your files type: Integrity\n\n";



    format!{ "{} {} {} {} {} {} {}",
                                    horizontal_line,
                                    header,
                                    exit,
                                    upload,
                                    download,
                                    integrity,
                                    horizontal_line
    }

}
