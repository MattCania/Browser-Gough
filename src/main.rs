mod url;
use std::io;

fn main() {
    let http_url = input_http();

    println!("| Welcome to Smoove Browser |");
    url::parse(&http_url);

}   

fn input_http() -> String {
    let mut url = String::new();
    println!("Enter a valid URL: ");

    io::stdin()
        .read_line(&mut url)
        .expect("Failed to receive input");

    return url;
}


    

