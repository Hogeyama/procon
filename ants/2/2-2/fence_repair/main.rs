
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use std::cmp;
use std::fmt;
use std::str;
use std::result;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

// Ita
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
struct Ita {//{{{
    len: i32,
    id:  i32,
}
impl PartialOrd<Ita> for Ita {
    fn partial_cmp(&self, other: &Ita) -> Option<Ordering> {
        // BinaryHeapはmax-heap
        // 最小要素を取り出したいので逆順にする
        let self_  = (self.len, self.id);
        let other_ = (other.len, other.id);
        Some(other_.cmp(&self_))
    }
}
impl Ord for Ita {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}//}}}
fn merge(x: Ita, y: Ita) -> Ita {//{{{
    Ita {
        len: x.len+y.len,
        id: x.id,
    }
}//}}}

// State
#[derive(Debug,Clone)]
struct S {//{{{
    n: i32,
    p: BinaryHeap<Ita>,
}//}}}
impl S {//{{{
    pub fn new() -> S {
        let mut r = StdinReader::new();
        let     n = r.get();
        let mut p = BinaryHeap::new();
        for i in 0..n {
            let x = r.get();
            p.push(Ita{ len:x, id:i });
        }
        S { n:n, p:p }
    }
    pub fn main(&mut self) -> i32 {
        let mut ans = 0;
        while self.p.len() > 1 {
            let min1 = self.p.pop().unwrap();
            let min2 = self.p.pop().unwrap();
            ans += min1.len + min2.len;
            self.p.push(merge(min1,min2));
        }
        ans
    }
}//}}}

fn main() {
    let mut s = S::new();
    println!("{:?}", s.main());
}

//{{{ ////////////////////////// Template /////////////////////////////////////
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
