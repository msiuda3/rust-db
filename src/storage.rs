
pub fn save(key: &str, value: &str){
    println!("Saving key: {}, value: {}", key, value);

}

pub fn get(key: &str) -> Option<String>{
    Some("test".to_string())
}