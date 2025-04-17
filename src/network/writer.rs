

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