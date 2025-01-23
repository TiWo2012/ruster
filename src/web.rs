use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

#[allow(unused_assignments)]
fn handle_requests(request: String) -> String {
    let mut response = String::new();
    let content_type: &str;
    
    if request.starts_with("GET /index.html") || request.starts_with("GET /") {
        content_type = "text/html";
        let html_content = read_file("src/index.html").unwrap_or_else(|e| {
            eprintln!("Failed to read HTML file: {}", e);
            return "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to read HTML file.".to_string();
        });
        response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}", content_type, html_content);
    } else if request.starts_with("GET /script.js") {
        content_type = "application/javascript";
        let js_content = read_file("src/script.js").unwrap_or_else(|e| {
            eprintln!("Failed to read JS file: {}", e);
            return "HTTP/1.1 500 Internal Server Error\r\n\r\nFailed to read JS file.".to_string();
        });
        response = format!("HTTP/1.1 200 OK\r\nContent-Type: {}\r\n\r\n{}", content_type, js_content);
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\nResource not found.".to_string();
    }

    return response;
}

pub fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    
    // Read the request from the client
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from stream: {}", e);
        return;
    }

    // Determine the requested resource
    let request = String::from_utf8_lossy(&buffer);

    // Write the Response back to the client
    if let Err(e) = stream.write_all(handle_requests(request.to_string()).as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }
}
