use byteorder::{ByteOrder, BigEndian};
use network::reader::{GetMessage, Operation, PutMessage};
use std::io::{self, Read, Write};
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::future::IntoFuture;

mod storage;
mod network;

const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;
const OPERATION_GET_RESPONSE: u8 = 0x81;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").await?;
    println!("Server is listening on port 7878...");
    loop{
        let (mut stream, addr) = listener.accept().await?;
        println!("New connection from {:?}", addr);

        tokio::spawn(async move{
            if let Err(e) = handle_message(&mut stream).await {
                eprintln!("Error occured in connection from: {:?}", e);
            }
        });
    }
    Ok(())
}


async fn handle_message(stream: &mut TcpStream) -> io::Result<()>{
    let result: Result<network::reader::Operation, network::reader::MessageError> = network::reader::read(stream).await;
    match result {
        Ok(network::reader::Operation::Get(get_message)) => {
                        println!("Handling GET operation");
                        handle_get(&get_message, stream);
                    }
        Ok(network::reader::Operation::Put(put_message)) => {
                        println!("Handling PUT operation");
                        handle_put(&put_message, stream);
                    }
        Err(_) => todo!(), //TODO handle case when no matchng operation was found
    }
    Ok(())
}

fn handle_get(get_message: &GetMessage, stream: &mut TcpStream){
    let mut value: Option<String> = storage::get(&get_message.key);
    println!("Received GET request for key: {}, value: {}", get_message.key, value);
    network::writer::write_get_answer(stream, true, &value.get_or_insert("".to_string()));
    println!("RETRIEVED SUCCESFULLY");
}

fn handle_put(put_message: &PutMessage, stream: &mut TcpStream){
    storage::save(&put_message.key, &put_message.value);
    println!("Received PUT request for key: {}, value: {}", put_message.key, value);
    network::writer::write_get_answer(stream, true, &put_message.value); //TODO write different response for PUT requests
    println!("SAVED SCUCCESFULLY");
}

