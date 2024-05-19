use crate::productions::get_productions;
use crate::productions::get_parsing_table;

pub fn syntatic_analyser(){
    // Your syntatic analiser implementation here
    let productions = get_productions();
    let parsing_table = get_parsing_table();
    
}