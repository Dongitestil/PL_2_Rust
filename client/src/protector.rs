extern crate rand;
use rand::prelude::*;

pub fn get_hash_str() -> String {
    let mut hash = String::with_capacity(5);
    let mut rng = rand::thread_rng();
    for _i in 0..5 {
        let r:f64 = rng.gen();
        let r1 = 6.0*r+1.0;
        let r1 = r1 as i64;
        let rstr = r1.to_string();
        hash += &rstr;
    }
    hash
}

pub fn get_session_key() -> String {
    let mut result = String::with_capacity(10);
    let mut rng = rand::thread_rng();
    for _i in 1..11 {
        let mut s = 1;
        let r:f64 = rng.gen();
        let r1 = 9.0*r+1.0;
        let r1 = r1 as i64;
        let rstr = r1.to_string();
        for c in rstr.chars(){
            if s==1 {
                let f=c;
                result.push(f); 
            }
            s +=1;
        }
    }
    result
}

pub fn verify_hash(hash: &str) ->bool {
    if hash.len() !=5 {
        return false
    }
    for c in hash.chars() {
        if !c.is_ascii_digit() {
            return false
        }
    }
    return true
}

pub fn next_session_key(hash: &str,session_key: &str) -> String {
    if !verify_hash(hash) {return "Error".to_string()}
    let mut result = 0;
    for c in hash.chars() {
        let f = c.to_string();
        let val = f.parse::<u64>().unwrap();
        result += calc_hash(session_key.to_string(), val).parse::<u64>().unwrap();;
    }
    return result.to_string()
}

pub fn calc_hash(session_key:String,val:u64) ->String{
    let mut result = String::new();
    match val {
        1 => {
            let slice = &session_key[0..5];
            let key_slice = slice.parse::<u64>().unwrap();
            let key_slice_str = "00".to_string()+&(key_slice % 97).to_string();
            let key_part = &key_slice_str[key_slice_str.len()-2..key_slice_str.len()];
            let key_int = key_part.parse::<u64>().unwrap();
            key_int.to_string()
        }
        2 => {
           let key_rev = session_key.chars().rev().collect::<String>();
            for c in key_rev.chars() {
                let f=c;
                result.push(f); 
            }
            result
        }
        3 => {
            let slice1 = &session_key[5..session_key.len()];
            let slice2 = &session_key[0..5];
            slice1.to_string() + &slice2.to_string()
        }
        4 => {
            let keypart = &session_key[1..session_key.len()-1];
            let mut num = 0;
            for c in keypart.chars() {
                let f:String = c.to_string();
                num += f.parse::<u64>().unwrap()+41;
            }
            num.to_string()
        }
        5 => {
            let mut num = 0;
            for c in session_key.chars() {
                let cc = (c as u8) ^ 43;
                let ccc = cc as char;
                if !ccc.is_ascii_digit() {
                    let cstr = (ccc as u8).to_string();
                    num += cstr.parse::<u64>().unwrap();
                } else {
                    let f = ccc.to_string();
                    num += f.parse::<u64>().unwrap();
                    }
            }
            num.to_string()
        }
        _ => {
            let key_int = session_key.parse::<u64>().unwrap()+val;
            key_int.to_string()
        }
    }
}