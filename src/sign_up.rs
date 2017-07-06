use file_io;

pub fn sign_up_service(data:String) -> String {

    let path = "database/user-credentials.txt";
    let (username, password) = split_credentials(data);
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


fn split_credentials(data:String) -> (String, String)  {

    let data_vector: Vec<&str> = data.split("--").collect();
    let (username,password) = (data_vector[0].to_owned(), data_vector[1].to_owned());

    (username,password)
}
