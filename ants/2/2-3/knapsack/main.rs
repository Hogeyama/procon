
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use std::cmp;
use std::fmt;
use std::str;
use std::result;

const MAX_N: usize = 100;
const MAX_W: usize = 100;
const MAX_V: usize = 100;
const MAX_LIMIT: usize = 1000;

struct Mono {
    w: i32,
    v: i32,
}

struct S {
    n     : usize,
    limit : usize,
    ws    : [usize; MAX_N],
    vs    : [i32; MAX_N],
    dp    : [[i32; MAX_N]; MAX_LIMIT],
}
impl S {
    pub fn new() -> S {
        let mut r     = StdinReader::new();
        let mut ws    = [0; MAX_N];
        let mut vs    = [0; MAX_N];
        let     dp    = [[-1; MAX_N]; MAX_LIMIT];
        let     n     = r.get();
        let     limit = r.get();
        for i in 0..n as usize {
            ws[i] = r.get();
            vs[i] = r.get();
        }
        S { n:n, limit:limit, ws:ws, vs:vs, dp:dp }
    }

    pub fn main(mut self) -> i32 {
        let limit = self.limit;
        self.rec(0,limit)
    }

    // 漸化式で解く場合
    pub fn main2(mut self) -> i32 {
        // iは大きい方から, wは小さい方から決まっていく
        for i in (0..self.n+1).rev() {
            for w in 0..self.limit+1 {
                let wi = self.ws[i];
                let vi = self.vs[i];
                self.dp[i][w] =
                    if i == self.n {
                        0
                    } else if w < self.ws[i] {
                        self.dp[i+1][w]
                    } else {
                        cmp::max(
                            self.dp[i+1][w],
                            self.dp[i+1][w-wi] + vi
                            )
                    };
            }
        }
        self.dp[0][self.limit]
    }

    // i-1番目までは決まっている, 残りをw以下で済ませたい
    fn rec(&mut self, i: usize, w:usize) -> i32 {
        if self.dp[i][w] >= 0 {
            return self.dp[i][w]
        }
        let res: i32;
        if i == self.n {
            res = 0;
        } else if w < self.ws[i] {
            res = self.rec(i+1,w);
        } else {
            let wi = self.ws[i];
            let vi = self.vs[i];
            res = cmp::max(
                    self.rec(i+1,w),
                    self.rec(i+1,w-wi) + vi
                    );
        }
        self.dp[i][w] = res;
        res
    }
}

fn main() {
    let s = S::new();
    //println!("{:?}", s.main());
    println!("{:?}", s.main2());
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
