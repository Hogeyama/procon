
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;

use std::cmp;
use std::result;
use std::str;
use std::fmt;

// macros
macro_rules! otry {//{{{
    ($e:expr) => (match $e { Some(e) => e, None => return None })
}//}}}

// read from stdin
struct StdinReader {//{{{
    words: Vec<String>, // 現在の行でまだ読んでいないwords
}//}}}
impl StdinReader {//{{{
    pub fn new() -> StdinReader {
        StdinReader { words: vec![] }
    }

    pub fn word_safe(&mut self) -> Result<String,String> {
        while self.words.len() == 0 {
            self.words = Self::to_words(try!(Self::getline()));
            self.words.reverse();
        }
        self.words.pop().ok_or_else(||panic!("Impossible"))
    }
    pub fn word(&mut self) -> String {
        self.word_safe().unwrap()
    }

    pub fn get_safe<T : str::FromStr>(&mut self) -> Result<String, String> {
        self.word_safe().and_then(Self::parse)
    }
    pub fn get<T : str::FromStr>(&mut self) -> T
        where <T as str::FromStr>::Err: fmt::Debug {
        Self::parse(self.word()).unwrap()
    }

    // supporting functions
    fn getline() -> Result<String, String> {
        let mut s = String::new();
        match io::stdin().read_line(&mut s) {
            Err(e) => Err(format!("{:?}", e)),
            Ok(n) if n == 0 => Err("StdinReader::getline(): no input".to_owned()),
            Ok(n) => Ok(s),
        }
    }
    fn to_words(s: String) -> Vec<String> {
        s.split_whitespace().map(|w|w.to_owned()).collect()
    }
    fn parse<T : str::FromStr>(s: String) -> Result<T,String> 
        where <T as std::str::FromStr>::Err: std::fmt::Debug {
        s.parse().map_err(|e| format!("{:?}", e))
    }

    //  TODO
    //  一行の途中までword()で読んだ状態でその行の残りを取得したい
    //      (String/Vec<String>で)
    //  iterの実装
}//}}}

///////////////////////////////////////////////////////////////////////////////
// Templateここまで
///////////////////////////////////////////////////////////////////////////////

fn main() {
    let mut r   = StdinReader::new();
    let     n   = r.get::<i32>();
    let mut s   = r.word().chars().collect::<Vec<_>>();
    let mut rs  = s.clone(); rs.reverse();
    let mut t   = Vec::new();
    for _ in 0..s.len() {
        if s < rs {
            t.push(pop_front(&mut s));
            rs.pop();
        } else {
            t.push(pop_front(&mut rs));
            s.pop();
        }
    }
    println!("{}",
        <String as std::iter::FromIterator<char>>::from_iter(t.into_iter())
    );
}

fn pop_front<T: Clone>(v: &mut Vec<T>) -> T {
    let x = v.first().unwrap().clone();
    v.remove(0);
    x
}

