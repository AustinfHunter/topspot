use crate::http;

use std::{fs,io::{prelude::*, BufReader, Write},net::{TcpListener, TcpStream}};

pub fn init(port : &str, log : &mut dyn Write) -> Result<(),std::io::Error> {
   let mut addr : String = String::from("127.0.0.1:");
   addr.push_str(port);

   let server = http::HttpServer::new(addr)?;
   println!("Listening on port {port}"); 
   for request in server.incoming_requests() {
       let req = request.0;
       let mut w = request.1;
       let r : http::Request;
       if req.is_ok() {
           r = req.unwrap();
       } else {
           continue;
       }
      let _ = w.write_all(b"HTTP/1.1 200 OK\r\n\r\n");
       println!{"Method: {}\nURI: {}\nProtocol: {}\nRequest Body: {}",r.method,r.uri,r.protocol,r.body};
   }
   Ok(())
}
