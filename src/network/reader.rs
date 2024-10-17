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
    let version: u8 = buffer[0];
    let operation: u8 = buffer[1];
}