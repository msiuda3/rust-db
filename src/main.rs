use byteorder::{ByteOrder, BigEndian};
use network::reader::Operation;
use std::io::{self, Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

mod storage;
mod network;

const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;
const OPERATION_GET_RESPONSE: u8 = 0x81;

fn main() -> io::Result<()> {
    let listener: <<Result<TcpListener, io::Error> as IntoFuture>::Output as Try>::Output = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Server is listening on port 7878...");
    loop{
        let (mut stream, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);

        tokio::spawn(async move{
            if let Err(e) = handle_client(&mut stream).await {
                eprintln!("Error occured in connection from: {:?}", e);
            }
        })
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer: Vec<u8> = vec![0; 256]; 
    let bytes_read = stream.read(&mut buffer)?;

    if bytes_read < 4 {
        eprintln!("Received message is too short.");
        return Ok(());
    }

    let version: u8 = buffer[0];
    let operation_type: u8 = buffer[1];
    let key_length: usize = buffer[2] as usize;

    if version != VERSION {
        eprintln!("Invalid version or operation type.");
        return Ok(());
    }


    if operation_type == OPERATION_GET{

        let key: &[u8] = &buffer[3..3 + key_length];
    let key_str = String::from_utf8_lossy(key);

        println!("Received GET request for key: {}", key_str);
        let mut value = storage::get(&String::from_utf8_lossy(key));
        let response = create_response(true, &value.get_or_insert("not_found".to_string()));
        stream.write_all(&response)?;

    }
    else if operation_type == OPERATION_PUT{
        let key: &[u8] = &buffer[4..4 + key_length];
        let key_str = String::from_utf8_lossy(key);

        let val_length: usize = buffer[3] as usize;
        let value = &buffer[4+key_length..4+key_length+val_length];
        let value_str = String::from_utf8_lossy(value);
        println!("Received PUT request for key: {}, value: {}", key_str, value_str);
        storage::save(&key_str, &value_str);
        let response = create_response(true, &value_str);
        stream.write_all(&response)?;
    }
    println!("ENDING CONNECTION");
    Ok(())
}

fn handle_message(stream: &mut TcpStream) -> io::Result{
    let result: Result<network::reader::Operation, network::reader::MessageError> = network::reader::read(stream);
    match result {
        network::reader::Operation(operation) => {
            match operation {
                Operation::Get(get_message) => {
                    println!("Handling GET operation");
                    handle_get(get_message, stream);
                }
                Operation::Put(put_message) => {
                    println!("Handling PUT operation");
                    handle_put(put_message, stream);
                }
            }
        }
    }
}

fn handle_get(getMessage: GetMessage, stream: &mut TcpStream){
    let mut value: Option<String> = storage::get(getMessage.key);
    println!("Received GET request for key: {}, value: {}", getMessage.key, value);
    network::writer::write_get_answer(stream, found, value);
}

fn handle_put(put_message: PutMessage, stream: &mut TcpStream){

}


fn create_response(found: bool, value: &str) -> Vec<u8> {
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
