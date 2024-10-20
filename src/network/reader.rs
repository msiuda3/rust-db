const VERSION: u8 = 0x01;
const OPERATION_GET: u8 = 0x01;
const OPERATION_PUT: u8 = 0x02;
const OPERATION_GET_RESPONSE: u8 = 0x81;

struct GetMessage{
    key: String,
}

struct PutMessage{
    key: String,
    value: String
}

enum Operation {
    Get(GetMessage),
    Put(PutMessage)
}

fn read(stream: TcpStream) -> Operation {
    let mut buffer: Vec<u8> = vec![0; 256]; 
    let bytes_read = stream.read(&mut buffer).await?;
    let version: u8 = buffer[0];
    let operation: u8 = buffer[1];

    match operation {
        OPERATION_GET => return handle_get(buffer),
        OPERATION_PUT => return handle_put(buffer)
    }

}

fn handle_get(buffer: Vec<u8>) -> Operation::Get {

}
fn handle_put(buffer: Vec<u8>) -> Operation::Put {

}