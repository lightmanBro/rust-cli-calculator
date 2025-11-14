use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
// use colored::*;
fn main() {
    // Bind to localhost but port = 0
    // Port 0 tells the OS: "assign any free port available"
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random port");
    
     // Retrieve the actual assigned address (IP + port)
    let adrr = listener.local_addr().expect("Failed to load local server");
    println!("🚀 listening on port http://{}",adrr);

    //listening for incoming data/ request
    // The server loop:
    // - Accept incoming TCP connections
    // - `incoming()` returns an iterator over connection attempts
    for stream in listener.incoming(){
        match stream {
            // Successfully accepted a TCP connection
            Ok(stream)=> {handle_connection(stream)},
            // If something fails, print the error
            Err(err)=> eprintln!("Error {}",err)
        }
    }
}

fn handle_connection(mut stream: TcpStream){
     // Create a buffer to store the incoming request (raw bytes)
    let mut buffer = [0;1024];

    //Read data from the TCP stream into a buffer
    //This reads the http requests sent by the server
    stream.read(&mut buffer).expect("Unable to read incoming data");

    //Read and load public/index.html from the public folder as a string
    let contents = fs::read_to_string("public/index.html").expect("Failed to read html file");

    //Build a basic HTTP response;
    // - Status line (HTTP/1.1 200 OK)
    // - Headers (Content-Length, Content-Type)
    // - Blank line (signals end of headers)
    // - Body (HTML file content)
    let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/html\r\n\r\n{}",
    contents.len(),
    contents
);
    // Write the response bytes back to the client
    stream.write(response.as_bytes()).unwrap();

    //Ensure everything is sent immediately.
    stream.flush().unwrap();

}