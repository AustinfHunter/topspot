use std::{
    io::{BufReader, BufRead},
    collections::HashMap, 
    net::{
        TcpStream,
        ToSocketAddrs,
        TcpListener,
    },
};

/// An enum for http response status headers - have not implemented anything related to writing
/// responses yet
pub enum Status {
    OK,
    NotFound,
    Forbidden,
    InternalServerError,
    NotImplemented,
    ServiceUnaivailable,
    HttpVersionNotSupported
}

/// HttpErr defines a set of errors - might extend functionality to make them implement
/// std::error::Erro and std::io::Errror to make error handling even easier
#[derive(Debug)]
pub enum HttpErr {
    ErrBadRequest(String),
    ErrBadHeaders(String),
    ErrParse(String),
    ErrUnknown(String),
}

/// Implements From<&str>, this allows certain errors to be bubbled up into an HttpErr
impl From<&str> for HttpErr {
    fn from(s: &str) -> Self {
        let e = s.split(':').collect::<Vec<&str>>();
        match e[0] {
            "HttpParseError" => HttpErr::ErrParse(String::from("{e[0]}:{e[1]}")),
            _ => HttpErr::ErrUnknown(String::from("{e[0]}:{e[1]}")),
        }
    }
}


/// IncomingRequest is an iterator over the incoming tcp streams - see std::io::TcpStream::Incoming
pub struct IncomingRequest<'a> {
    listener: &'a TcpListener,
}

/// Implements the Iterator trait for IncomingRequest
impl <'a> Iterator for IncomingRequest<'a> {
    type Item =(Result<Request,HttpErr>,TcpStream);
    fn next(&mut self) -> std::option::Option<(std::result::Result<Request,HttpErr>,TcpStream)> {
        Some(Request::from_tcp_stream(self.listener.accept().ok()?.0))
    }
}

/// HttpServer defines a simple http server, which just wraps a TcpListener and allows for parsing
///requests
pub struct HttpServer<'a> {
    listener: TcpListener,
    log: Option<&'a mut dyn std::io::Read>,
}

/// Implements the simple http server - minimally functional at the moment, will be extended
impl HttpServer<'_>  {
    //! Returns a new HttpServer, 
    pub fn new<A: ToSocketAddrs>(addr: A) -> Result<Self,std::io::Error> {
        Ok(Self{listener: TcpListener::bind(addr)?,log: None})
    }

    pub fn start() {}

    pub fn register_handler() {}

    pub fn use_logs() {}

    pub fn incoming_requests(&self) -> IncomingRequest {
        IncomingRequest{listener: &self.listener}
    }
}

/// Header defines a type for http headers
pub type Header = (String, [String]);

/// Headers defines a type that is used to store multiple http headers - HashMap used for efficient
/// lookups
pub type Headers = HashMap<String,Vec<String>>;

/// Request defines a simple structure to represent a parsed http request
pub struct Request {
    pub method: String,
    pub uri: String,
    pub protocol: String,
    headers: Headers,
    pub body: String,
}

/// Implementation of the Request type - will be extended soon
impl Request {
    /// from_tcp_stream parses a TcpStream and returns a new instance of a Request or an HttpErr
    pub fn from_tcp_stream(mut stream: TcpStream) -> (Result<Request,HttpErr>,TcpStream) {
        let mut buf = BufReader::new(&mut stream);
        (Self::parse_request(&mut buf),stream)
    }

    /// parse_request parses a raw http request and returns an instance of a Request or an HttpErr
    fn parse_request(buf: &mut dyn BufRead) -> Result<Request,HttpErr>{
        let method: String;
        let uri: String;
        let protocol: String;
        let mut headers: Headers = Headers::new();
        let mut body: String;

        let mut lines = buf.lines();
        let mut req_line = lines.next().unwrap().unwrap();
        let mut req_vals = req_line.split(' ');
        method = req_vals.next().ok_or("HttpParseError: Could not parse method")?.to_string();
        uri = req_vals.next().ok_or("HttpParseError: Could not parse uri")?.to_string();

        protocol = req_vals.next().ok_or("HttpParseError: Could not parse protocol")?.to_string();

        let mut b_lines : Vec<String> = Vec::new();
        let mut in_body = false;
        loop {
            let mut line = String::new();
            let l = buf.read_line(&mut line).unwrap();
           
            let pair = line.split(':').collect::<Vec<&str>>();
            if l < 1 || pair.len() < 2 {
                break;
            }
            headers.insert(String::from(pair[0]),pair[1].split(',').map(|a| String::from(a)).collect::<Vec<String>>());
        }
        let size : usize;
        if headers.contains_key("Content-Length") {
                let content_len = headers.get("Content-Length").unwrap();
                size = content_len[0].trim().parse::<usize>().unwrap();
        } else {
            size = 0;
        }
        let mut body_buf = vec![0;size];
        let _ = buf.read_exact(&mut body_buf);
        body = body_buf.into_iter().fold(String::new(), |s,l| s + String::from_utf8(l.to_be_bytes().to_vec()).unwrap().as_str());
        Ok(Request{method,uri,protocol,headers,body})
    
    }
}
