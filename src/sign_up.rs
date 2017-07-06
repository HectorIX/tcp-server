use file_io;
use parser;


pub fn sign_up_service(data:String) -> String {

    let path = "database/user-credentials.txt";
    let (username, password) = parser::split_credentials(data);
    let mut credentials = "<**>\n".to_owned();

    credentials.push_str("username:=");
    credentials.push_str(&username);
    credentials.push_str("\n");
    credentials.push_str("password:=");
    credentials.push_str(&password);
    credentials.push_str("\n");

    file_io::write_file(path.to_string(), credentials.to_owned());

    format!("Congradulation {}! You signed up successfully!\n", username)
}
