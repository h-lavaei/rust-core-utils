use std::fs;

// equivalent of cat
pub fn rat(path:&str) -> String{
    let mut result = String::new();
    let file = fs::read_to_string(path);
    if file.is_ok() {
        result = file.unwrap().to_string();
    }else {
        result = file.err().unwrap().to_string();
    }
    result
}