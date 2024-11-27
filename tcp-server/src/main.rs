//! Single threaded TCP Server.
//!
//! ## Intoduction
//!
//! Make intoduction as crate gets more complicated.
//!
//! ## Features
//!
//! - [x] Single thread server
//! - [x] HttpRequest struct deserialization from raw requests
//! - [x] Response generation from files
//! - [x] Route handling with respect to method, path, and body
//! - [ ] Request query string parsed
//! - [ ] Multithread with pooling
//! - [ ] Database interface
//!

use core::str;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use url::form_urlencoded;

//should create a series of interconnected state machines using enums, states, and matches
/*EG
type Prog = Vec<Command>;

enum Type {
    Int(i32),
    String(String),
    Address(Address),
}
enum Command {
    LD(Type),
    ADD,
    MULT,
    DUP
}*/

//Result generalization, could replace String with custom error enum
type Result<T> = core::result::Result<T, String>;

//methods the server allows along with a catchall Error
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Error,
}

#[allow(unused)]
impl HttpMethod {
    //const of all possible types to all iteration over all possible values
    const ALL_TYPES: [HttpMethod; 5] =
        [Self::Get, Self::Post, Self::Put, Self::Delete, Self::Error];

    //Returns the
    fn to_string(&self) -> String {
        match self {
            Self::Get => "GET".to_string(),
            Self::Post => "POST".to_string(),
            Self::Put => "PUT".to_string(),
            Self::Delete => "DELETE".to_string(),
            Self::Error => "ERROR".to_string(),
        }
    }

    fn from_string(string: String) -> Self {
        for method in Self::ALL_TYPES {
            if string.starts_with(&method.to_string()) {
                return method;
            }
        }
        HttpMethod::Error
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    fn from_bytes(bytes: &[u8]) -> Self {
        for method in Self::ALL_TYPES {
            if bytes.starts_with(&method.to_bytes()) {
                return method;
            }
        }
        HttpMethod::Error
    }
}

struct HttpRequest {
    method: HttpMethod,
    path: String,
    parameters: Option<Vec<(String, String)>>,
    body: Option<String>,
}

#[allow(unused)]
impl HttpRequest {
    const DEFAULT_404_HTML: &str = "<html><body><h1>404 Not Found</h1></body></html>"; //TODO: Move to HttpResponse

    fn to_string(&self) -> String {
        format!(
            "{} {}{} HTTP/1.1\r\nContent Length: {}\r\n\r\n{}",
            self.method.to_string(),
            self.path,
            self.parameters_to_string(),
            self.body.as_ref().unwrap_or(&String::new()).len(),
            self.body.as_ref().unwrap_or(&String::new()),
        )
    }

    //Returns a String in the format of "?key1=val1&keyN=valN" or "" if parameters is empty
    fn parameters_to_string(&self) -> String {
        self.parameters
            .as_ref()
            .filter(|params| !params.is_empty())
            .map(|params| {
                format!(
                    "?{}",
                    params
                        .iter()
                        .map(|(key, val)| format!("{key}={val}"))
                        .collect::<Vec<String>>()
                        .join("&")
                )
            })
            .unwrap_or_default()
    }

    fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    fn from_request_bytes(buffer: &[u8]) -> Self {
        //split the request on delimiter to separate the header and body, on err assume no body and set whole buffer to header
        let delimiter = b"\r\n\r\n";
        let (header, body) = buffer.split_at(
            buffer
                .windows(delimiter.len())
                .position(|window| window == delimiter)
                .unwrap_or_else(|| buffer.len()),
        );

        //split the header on spaces ' '
        let (method, whole_path) = {
            let mut split = header.splitn(3, |&byte| byte == b' ');
            (
                HttpMethod::from_bytes(split.next().unwrap_or_default()),
                split.next().unwrap_or_default(),
            )
        };

        let (path, parameters): (String, Option<Vec<(String, String)>>) = {
            //split array at pos of '?' or end of byte array
            let (path_bytes, query_string_bytes) = whole_path.split_at(
                whole_path
                    .iter()
                    .position(|&b| b == b'?')
                    .unwrap_or(whole_path.len()),
            );

            (
                str::from_utf8(path_bytes).unwrap_or_default().to_string(),
                Some(
                    form_urlencoded::parse(if query_string_bytes.is_empty() {
                        query_string_bytes
                    } else {
                        &query_string_bytes[1..]
                    })
                    .into_owned()
                    .collect(),
                ),
            )
        };

        HttpRequest {
            method,
            path,
            parameters,
            body: (body.len() > delimiter.len())
                .then(|| String::from_utf8_lossy(&body[delimiter.len()..]).into_owned()),
        }
    }

    fn error() -> Self {
        HttpRequest {
            method: HttpMethod::Error,
            path: String::new(),
            body: None,
            parameters: None,
        }
    }

    fn test() {
        HttpRequest::new(HttpMethod::Get, "/", Some([]), None) //be able to pass arr ([]) or String as well?
    }

    fn new(
        method: HttpMethod,
        path: String,
        parameters: Option<Vec<(String, String)>>,
        body: Option<String>,
    ) -> Self {
        HttpRequest {
            method,
            path,
            parameters,
            body,
        }
    }

    fn default_get() -> Self {
        HttpRequest::get("/".to_string(), None, None)
    }

    fn default_post() -> Self {
        HttpRequest::post("".to_string(), None, None)
    }

    fn get(path: String, parameters: Option<Vec<(String, String)>>, body: Option<String>) -> Self {
        HttpRequest {
            method: HttpMethod::Get,
            path,
            parameters,
            body,
        }
    }

    fn post(path: String, parameters: Option<Vec<(String, String)>>, body: Option<String>) -> Self {
        HttpRequest {
            method: HttpMethod::Post,
            path,
            parameters,
            body,
        }
    }
}

#[allow(unused)]
enum Address {
    IPv4(String),
    IPv6(String),
}

impl Address {
    //Returns reference to internal enum String, remove '&' from '&self' and '&String' to return owned String
    fn to_string(&self) -> &String {
        match self {
            Address::IPv4(addr) | Address::IPv6(addr) => addr,
        }
    }
}

//Returns a tcp listener on success or error string on failure
fn init_server(address: Address) -> Result<(TcpListener, Address)> {
    let addr_str = address.to_string();
    Ok((
        TcpListener::bind(addr_str).map_err(|error| {
            format!("Failed to bind server at address {addr_str}, Error: {error}")
        })?,
        address,
    ))
}

//Forever wait for connections on the listener
fn wait_for_connections(listener: TcpListener) {
    listener
        .incoming()
        .for_each(|stream_result| match stream_result {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(error) => {
                eprintln!("Error occured when establishing connection. Error: {error}");
            }
        });
}

fn handle_connection(mut stream: TcpStream) {
    //allocate a buffer
    let mut buffer = [0; 1024]; //TODO: change size later

    //read the request from the stream to the buffer //TODO: add stream identifier for message
    if let Err(error) = stream.read(&mut buffer) {
        eprintln!("Failed to read from the stream. Error: {error}");
    }
    println!("Request size: {}", buffer.len());
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    //construct a request struct
    let request = HttpRequest::from_request_bytes(&buffer);

    //validate request credentials

    //construct response from possible pathways
    let root: String = "views/".to_string();
    let gen_res = |filename: &str| generate_response(root + filename);
    let response = match request {
        HttpRequest {
            method: HttpMethod::Get,
            path: p,
            body: _,
        } => match p.as_str() {
            "/" => gen_res("index.html"),
            "/page1" => gen_res("index.html"),
            "/page2" => gen_res("index.html"),
            _ => gen_res("404.html"),
        },
        /*HttpRequest {
            method: HttpMethod::Post,
            path: p,
            body: b,
        } => match p.as_str() {
            _ => gen_res("404.html"),
        },*/
        /*HttpRequest {
            method: HttpMethod::Put,
            path: p,
            body: b,
        } => match p.as_str() {
            _ => gen_res("404.html"),
        },*/
        /*HttpRequest {
            method: HttpMethod::Delete,
            path: p,
            body: _,
        } => match p.as_str() {
            _ => gen_res("404.html"),
        },*/
        _ => gen_res("404.html"),
    };

    //send generated response //TODO: add stream identifier for message
    if let Err(error) = send_response(stream, &response) {
        eprintln!("Failed to send response to stream. Error: {error}")
    }
}

fn send_response(mut stream: TcpStream, data: &[u8]) -> Result<()> {
    let len = data.len();
    let mut remaining_bytes = len;
    let mut head = 0;

    while remaining_bytes > 0 {
        match stream.write(&data[head..]) {
            Ok(0) => break,
            Ok(n) => {
                remaining_bytes -= n;
                head += n;
                if let Err(error) = stream.flush() {
                    return Err(format!(
                        "Failed to send data, sent {}/{} bytes. Error: {}",
                        len - remaining_bytes,
                        len,
                        error
                    ));
                }
            }
            Err(error) => {
                return Err(format!(
                    "Failed to send data, sent {}/{} bytes. Error: {}",
                    len - remaining_bytes,
                    len,
                    error
                ));
            }
        }
    }
    Ok(())
}

//TODO: make HttpResponse
fn generate_response(path: String) -> Vec<u8> {
    //read content file
    let (status, content) = match fs::read_to_string(&path) {
        Ok(content) => ("200 Ok", content),
        Err(error) => {
            eprintln!("Error reading file {path}. Error: {error}");
            ("404 Not Found", HttpRequest::DEFAULT_404_HTML.to_string())
        }
    };

    //construct header
    let header = format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n",
        status,
        content.len()
    );

    //construct response by appeding content to header
    format!("{header}\r\n\r\n{content}").into_bytes()
}

fn main() {
    let listener = match init_server(Address::IPv4("127.0.0.1:7878".to_string())) {
        Ok((tcp_listener, address)) => {
            println!("Server listening on '{}'", address.to_string());
            tcp_listener
        }
        Err(error) => {
            eprintln!("{error}");
            return;
        }
    };

    wait_for_connections(listener);
}
