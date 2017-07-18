use std::error::Error;
use std::io::prelude::*;
use std::fs::{OpenOptions};


// Write a message to the given file name.
// If the file does not extist, create it.
// The first argument (filename:String) is the name of the file
// and the second argument (message:String) is the message will be stored.
pub fn write_file( filename:String, context:String ) -> String {


    // Create the File if not already exists.
    let mut file = match OpenOptions::new()
                                    .read(true)
                                    .write(true)
                                    .create(true)
                                    .append(true)
                                    .open(&filename) {

        Err(failure) => { return format!("**Failed: {}", failure.description()); },
        Ok(file) => file,
    };

    // Write the context into the file.
    match file.write_all(context.as_bytes()) {

        Err(failure) => { return format!("**Failed {}", failure.description()); },
        Ok(_) => println!("Message successfully stored into {}.", filename),
    }

    "OK".to_string()
}



// Read the contents of the given file name and return them as a string.
// The argument (filename:String) is the name of the file
pub fn read_file( filename:String ) -> String {


    // Open the File.
    let mut file = match OpenOptions::new()
                                     .read(true)
                                     .open(&filename) {

        Err(failure) => { return format!("**Failed {}", failure.description()); },
        Ok(file) => file,
    };



    let mut context = String::new();

    // Read the file and store it in context.
    match file.read_to_string(&mut context) {

        Err(failure) => { return format!("**Failed {}", failure.description()) },
        Ok(_) => println!("Message successfully read from {}.", filename),
    };

    context

}
