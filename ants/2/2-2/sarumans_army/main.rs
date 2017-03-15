
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use std::cmp;
use std::result;
use std::str;
use std::fmt;

#[derive(Debug,Clone)]
struct S {//{{{
    pub n: i32,
    pub r: i32,
    pub p: Vec<i32>,
    pub marked: Vec<i32>,
}//}}}
impl S {//{{{
    pub fn new() -> Self {
        let mut stdin = StdinReader::new();
        let     n = stdin.get::<i32>();
        let     r = stdin.get::<i32>();
        let mut p = Vec::new();
        let     marked = Vec::new();
        for _ in 0..n {
            p.push(stdin.get::<i32>());
        }
        p.sort();
        p.reverse();
        S { n:n, r:r, p:p, marked:marked }
    }

    pub fn main(&mut self) -> Vec<i32> {
        let r = self.r;
        while let Some(x) = self.p.pop() {
            match pop_while_and_last(|&z| x+r>=z, &mut self.p) {
                None => {
                    self.marked.push(x);
                },
                Some(y) => {
                    self.marked.push(y);
                    pop_while_and_last(|&z| y+r>=z, &mut self.p);
                },
            }
        }
        self.marked.clone()
    }
}//}}}

fn main() {
    let mut s = S::new();
    println!("{:?}", s.main());
}


















//{{{ Template
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
// functions for vector
fn pop_while_and_last<T,P>(p: P, xs: &mut Vec<T>) -> Option<T>//{{{
        where P: Fn(&T) -> bool,
              T: std::fmt::Debug, {
    // pを満たす限りpopし続ける 最後にpopしたやつを返す
    let mut r = None;
    while let Some(y) = xs.pop() {
        if p(&y) {
            r = Some(y)
        } else {
            xs.push(y);
            break;
        }
    }
    r
}//}}}
//}}}
