
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

const N: usize = 100000;

struct IS {
    pub n: usize,
    //pub jobs: [Job; N],
    pub jobs: Vec<Job>,
}
#[derive(Debug,Clone,Copy,PartialEq)]
struct Job(i32,i32);
enum T { Start, End }


impl IS {
    fn new() -> IS {
        let mut r = StdinReader::new();
        let     n = r.get();
        //let mut jobs = [Job(0,0); N];
        let mut jobs = Vec::new();
        for i in 0..n {
            let s = r.get();
            let t = r.get();
            //jobs[i] = Job(s,t);
            jobs.push(Job(s,t));
        }
        IS { n:n, jobs:jobs }
    }
    fn main(&mut self) -> u32 {
        (&mut self.jobs[0..self.n]).as_mut()
            .sort_by(|a,b| a.1.cmp(&b.1));
        let mut cnt = 0;
        let mut e = -1;
        for i in 0..self.n {
            let Job(s,t) = self.jobs[i];
            if e < s {
                e = t;
                cnt += 1;
            }
        }
        cnt
    }
}

fn main() {
    let mut is = IS::new();
    println!("{:?}", is.main());
}


