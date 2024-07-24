use http_req::request;

fn main() {
    // Container for body of a response.
    let mut body = Vec::new();

    // Sends a HTTP GET request and processes the response. Saves body of the response to `body` variable.
    let res = request::get_with_update("https://www.rust-lang.org/learn", &mut body, |x| { println!("{x}") }).unwrap();

    //Prints details about the response.
    println!("Status: {} {}", res.status_code(), res.reason());
    println!("Headers: {}", res.headers());
    //println!("{}", String::from_utf8_lossy(&body));
}
