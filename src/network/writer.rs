use tokio::{io::AsyncWriteExt, net::TcpStream};

const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;
const OPERATION_GET_RESPONSE: u8 = 0x81;
const STATUS_FOUND: u8 = 0x00;
const STATUS_NOT_FOUND: u8 = 0x01;


pub fn write_get_answer(stream: &mut TcpStream, found: bool, value: &str) {
    let response = create_response(found, value);
   stream.write_all(&response); 
}


pub fn create_response(found: bool, value: &str) -> Vec<u8> {
    let mut response = vec![VERSION];

    response.push(OPERATION_GET_RESPONSE);
    response.push(if found { 0x00 } else { 0x01 });

    if found {
        response.push(value.len() as u8);
        response.extend_from_slice(value.as_bytes());
    } else {
        response.push(0x00); 
    }

    response
}