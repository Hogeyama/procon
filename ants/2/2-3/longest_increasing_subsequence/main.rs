
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use std::cmp;
use std::result;
use std::str;
use std::fmt;

const MAX_N: usize = 1000;
const MAX_A: usize = 1000000;

struct Hoge(Vec<i32>);

#[derive(Debug,Clone,PartialEq,Eq)]
struct Seq {
    len: i32,
    max: i32,
    body: Vec<i32>,
}
impl Seq {
    pub fn new(len:i32,max:i32,body:Vec<i32>) -> Seq {
        Seq { len:len, max:max, body:body }
    }
    pub fn single(a:i32) -> Seq {
        Seq { len:1, max:a, body:vec![a] }
    }
}
impl PartialOrd for Seq {
    fn partial_cmp(&self, other: &Seq) -> Option<cmp::Ordering> {
        //長いほうが良い
        match Ord::cmp(&self.len, &other.len) {
            //長さが同じならmaxが小さいほうがよい
            cmp::Ordering::Equal
                => Some(Ord::cmp(&other.max, &self.max)),
            r
                => Some(r),
        }
    }
}
impl Ord for Seq {
    fn cmp(&self, other: &Seq) -> cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

struct S {
    pub n:  usize,
    pub a:  [i32; MAX_N+1],
    pub dp: Vec<Seq>,
}
impl S {
    pub fn new() -> S {
        let mut r  = StdinReader::new();
        let     n  = r.get();
        let mut a  = [0; MAX_N+1];
        let mut dp = Vec::new();
        for i in 0..n+1 {
            a[i] = r.get();
            dp.push(Seq {len:-1, max:-1, body:Vec::new()});
            //Vec<i32>はコピーが使えないので仕方なく
        }
        S { n:n, a:a, dp:dp }
    }

    // dp[i+1] := a[0]~a[i]を使った最適解
    pub fn main(&mut self) -> i32 {
        self.dp[0] = Seq { len:0, max:-1, body:Vec::new()};
        for i in 0..self.n {
            if self.dp[i].max < self.a[i] {
                self.dp[i+1] = self.add(i,i);
            } else {
                //a[i+1]が追加できる最近のものを探す
                let k = (0..i+1).rev()
                        .find(|&j| self.dp[j].max < self.a[i])
                        .unwrap();
                self.dp[i+1] = cmp::max(self.dp[i].clone(), self.add(k,i));
            }
        };
        self.dp[self.n].len
    }

    // dp[i] = 最後がa[i]となる最適解 こっちのほうがスッキリする
    // 平均ループ回数は上のほうが少なそう 定数倍でどうなるか
    pub fn main2(&mut self) -> i32 {
        for i in 0..self.n {
            self.dp[i] = Seq::single(self.a[i]);
            for j in 0..i {
                if self.a[j] < self.a[i] {
                    self.dp[i] = cmp::max(self.add(j,i),self.dp[i].clone());
                }
            }
        }
        self.dp.iter().map(|ref seq|seq.len).max().unwrap()
    }

    // dp[i] = 長さがi+1の増加部分列における最終要素の最小値
    // TODO
    pub fn main3(&mut self) -> i32 {
        panic!("")
    }

    //dp[i].bodyにa_jを追加したもの
    fn add(&self, i: usize, j: usize) -> Seq {
        Seq::new(self.dp[i].len+1, self.a[j], push(&self.dp[i].body,self.a[j]))
    }
}


fn main() {
    let mut s = S::new();
    println!("n= {:?}", s.n);
    println!("Result: {:?}", s.main());
    //println!("Result: {:?}", s.main2());
    for i in 0..s.n+1 {
        println!("{}, {:?}", i, s.dp[i]);
    }
}

fn push<T: Clone>(v: &Vec<T>, x:T) -> Vec<T> {
    let mut u = v.clone();
    u.push(x);
    u
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

