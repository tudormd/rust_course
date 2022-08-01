fn main() {
    let username = option_env!("USERNAME").expect("Need to set 'USERNAME' environment variable");
    let password = option_env!("PASSWORD").expect("Need to set 'PASSWORD' environment variable");

    println!("Acces token {}", third_party_api(&username,&password));
}


fn third_party_api(username: &str, password: &str) -> String {
    let mut access_token = username.to_string();
    access_token.push_str(password.split_at(2).0);

    access_token
}
