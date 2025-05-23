use tokio::{io::AsyncReadExt, net::TcpStream};

const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;
const OPERATION_GET_RESPONSE: u8 = 0x81;

pub struct GetMessage{
    pub key: String,
}

pub struct PutMessage{
    pub key: String,
    pub value: String
}

pub enum Operation {
    Get(GetMessage),
    Put(PutMessage)
}

pub enum MessageError{
    InvalidOperation
}

pub async fn read(stream: &mut TcpStream) -> Result<Operation, MessageError> {
    let mut buffer: Vec<u8> = vec![0; 256]; 
    stream.read(&mut buffer).await;
    let version: u8 = buffer[0];
    let operation: u8 = buffer[1];

    match operation {
        OPERATION_GET => return Ok(handle_get(buffer)),
        OPERATION_PUT => return Ok(handle_put(buffer)),
        _ => return Err(MessageError::InvalidOperation)
    }

}

fn handle_get(buffer: Vec<u8>) -> Operation {
    let key_length: usize = buffer[3] as usize;
    let key_bytes: &[u8] = &buffer[3 .. key_length];
    return Operation::Get(GetMessage {key: string_from_bytes(key_bytes)});
}
fn handle_put(buffer: Vec<u8>) -> Operation {
    let key_length: usize = buffer[3] as usize;
    let key_bytes: &[u8] = &buffer[3 .. key_length];
    let key_str = string_from_bytes(key_bytes);

    let value_length_position = 3 + key_length;
    let value_length = buffer[value_length_position] as usize;
    let value_bytes= &buffer[value_length_position + 1 .. value_length_position + 1 + value_length];
    let value_str = string_from_bytes(&value_bytes);
    return Operation::Put(PutMessage {key: key_str, value: value_str});
}

fn string_from_bytes(value: &[u8]) -> String{
    return String::from_utf8_lossy(value).to_string();
}