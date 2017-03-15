
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use std::cmp;
use std::result;
use std::str;
use std::fmt;

use std::cmp::*;

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
///////////////////////////////////////////////////////////////////////////////

const MAX_N: usize = 100;
const MAX_M: usize = 100;

#[derive(Debug,Clone,Copy,PartialEq)]
enum Status {
    Wet,
    Dry,
}
use Status::*;

struct Solve {
    pub n: usize,
    pub m: usize,
    pub field: [[Status; MAX_N]; MAX_M],
    pub ans: i32,
}
impl Solve {
    pub fn new() -> Solve {
        let mut r = StdinReader::new();
        let n: usize = r.get();
        let m: usize = r.get();
        let mut field: [[Status; MAX_N]; MAX_M] = [[Dry; MAX_N]; MAX_M];
        let ans = 0;
        for i in 0..n {
            let word = r.word();
            for (j,c) in word.chars().enumerate() {
                if c == 'W' {
                    field[i][j] = Wet
                }
            }
        }
        Solve { n:n, m:m, field:field, ans:ans }
    }

    pub fn main(&mut self) {
        for i in 0..self.n {
            for j in 0..self.m {
                if self.field[i][j] == Wet {
                    //println!("main: {:?} is Wet", (i,j));
                    self.dfs(i,j);
                    self.ans += 1;
                }
            }
        }
    }

    pub fn dfs(&mut self, i: usize, j: usize) {
        if self.field[i][j] == Wet {
            self.field[i][j] = Dry;
            let pred = |x| if x == 0 { 0 } else { x-1 };
            // neighbors
            for p in pred(i)..min(i+1, self.n)+1 {
                for q in pred(j)..min(j+1, self.m)+1 {
                    self.dfs(p,q)
                }
            };
        }
    }

}



fn main() {
    let mut s = Solve::new();
    s.main();
    println!("{:?}", s.ans);
}


