
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::io;
use std::cmp;
use std::result;
use std::str;
use std::fmt;

use std::cmp::*;
use std::collections::VecDeque;

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
// ここから本体
///////////////////////////////////////////////////////////////////////////////

const MAX_N: usize = 100;
const MAX_M: usize = 100;
const INF: i32 = 2<<20;

#[derive(Debug,Clone,Copy,PartialEq)]
enum Status {
    Start,
    Goal,
    Wall,
    Path, // 名前
}
use Status::*;
#[derive(Debug,Clone,Copy)]
struct Pos(i32,i32);

struct Solve {
    n: usize,
    m: usize,
    field: [[Status; MAX_N]; MAX_M],
    d: [[i32; MAX_N]; MAX_M],
    queue: VecDeque<Pos>,
    ans: i32,
    start: Pos,
    goal: Pos,
}
impl Solve {
    pub fn new() -> Solve {
        let mut r = StdinReader::new();
        let n = r.get();
        let m = r.get();
        let queue = VecDeque::new();
        let d = [[INF; MAX_N]; MAX_M];
        let ans = 0;

        let mut start: Pos = Pos(-1,-1);
        let mut goal:  Pos = Pos(-1,-1);
        let mut field: [[Status; MAX_N]; MAX_M] = [[Path; MAX_N]; MAX_M];
        for i in 0..n {
            for (j,c) in r.word().chars().enumerate() {
                match c {
                    'S' => {
                        field[i][j] = Start;
                        start = Pos(i as i32, j as i32);
                    },
                    'G' => {
                        field[i][j] = Goal;
                        goal = Pos(i as i32, j as i32);
                    },
                    '#' => field[i][j] = Wall,
                    '.' => field[i][j] = Path,
                    _   => panic!(""),
                }
            }
        }
        Solve { n:n, m:m, d:d, queue:queue, field:field, ans:ans, start:start, goal:goal }
    }

    pub fn main(&mut self) -> i32 {
        self.bfs();
        self.d(self.goal)
    }

    pub fn bfs(&mut self) {
        let s = self.start;
        self.queue.push_back(s);
        self.set_d(s,0);
        while !self.queue.is_empty() {
            let p = self.queue.pop_front().unwrap();
            if self.field(p) == Goal { return }
            for np in Self::neighbor(p) {
                if self.d(np)==INF && self.field(np)!=Wall {
                    let nd = self.d(p)+1;
                    self.set_d(np,nd);
                    self.queue.push_back(np);
                }
            }
        }
    }

    fn neighbor(Pos(i,j): Pos) -> Vec<Pos> {
        vec![Pos(i+1,j),Pos(i,j+1),Pos(i-1,j),Pos(i,j-1)]
            .into_iter()
            .filter(|&Pos(i,j)| i>=0 && j>=0)
            .collect()
    }

    fn set_d(&mut self, Pos(i,j): Pos, x: i32) {
        self.d[i as usize][j as usize] = x
    }
    fn field(&self, Pos(i,j): Pos) -> Status {
        self.field[i as usize][j as usize]
    }
    fn d(&self, Pos(i,j): Pos) -> i32 {
        self.d[i as usize][j as usize]
    }
}

fn main() {
    println!("{:?}", Solve::new().main());
}


