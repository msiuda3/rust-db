use byteorder::{ByteOrder, BigEndian};
use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

mod storage;

const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;
const OPERATION_GET_RESPONSE: u8 = 0x81;

fn main() -> io::Result<()> {
    // Start the TCP listener on localhost and port 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    println!("Server is listening on port 7878...");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_client(&mut stream)?;
            }
            Err(e) => {
                eprintln!("Failed to accept a connection: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    // Buffer to read the incoming message
    let mut buffer = vec![0; 256]; // Adjust size as needed
    let bytes_read = stream.read(&mut buffer)?;

    // Check if we have enough bytes for the expected protocol format
    if bytes_read < 4 {
        eprintln!("Received message is too short.");
        return Ok(());
    }

    // Parse the incoming message
    let version: u8 = buffer[0];
    let operation_type: u8 = buffer[1];
    let key_length: usize = buffer[2] as usize;

    // Validate the version
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


fn create_response(found: bool, value: &str) -> Vec<u8> {
    let mut response = vec![VERSION];

    // Operation type for response
    response.push(OPERATION_GET_RESPONSE);
    // Status: 0x00 for found, 0x01 for not found
    response.push(if found { 0x00 } else { 0x01 });

    // Add value length and value
    if found {
        response.push(value.len() as u8);
        response.extend_from_slice(value.as_bytes());
    } else {
        response.push(0x00); // No value
    }

    response
}
