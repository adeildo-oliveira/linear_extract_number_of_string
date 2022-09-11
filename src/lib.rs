#![allow(unused)]
pub mod linear_search_extract_interger_number {
    pub fn return_number_base_64(str: &String) -> usize {
        let numbers: String = str.chars().filter(|caracter| caracter.is_digit(9)).collect();
    
        let numbers: usize = match numbers.trim().parse(){
            Ok(num) => num,
            Err(_) => 0,
        };
    
        numbers
    }
}
