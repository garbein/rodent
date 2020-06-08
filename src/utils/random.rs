use std::iter::FromIterator;
use rand::distributions::{Distribution, Uniform};

#[allow(dead_code)]
pub fn rand_str(len: i32) -> String {    
    let chars_set: Vec<char> = "023456789abcdefghijkmnpqrstuvwxyz".chars().collect();    
    let mut chars: Vec<char> = Vec::new();
    let range = Uniform::new(0, chars_set.len());    
    let mut rng = rand::thread_rng();
    for _ in 0 .. len {        
        let b = range.sample(&mut rng);        
        chars.push(chars_set[b]);   
    }    
    let s = String::from_iter(chars.into_iter());    
    s  
}

pub fn rand_num(len: i32) -> String {    
    let chars_set: Vec<char> = "01235667889".chars().collect();    
    let mut chars: Vec<char> = Vec::new();
    let range = Uniform::new(0, chars_set.len());    
    let mut rng = rand::thread_rng();
    for _ in 0 .. len {        
        let b = range.sample(&mut rng);        
        chars.push(chars_set[b]);   
    }    
    let s = String::from_iter(chars.into_iter());    
    s
}
