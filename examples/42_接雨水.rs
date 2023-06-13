/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */
struct Solution;

use std::fmt::Display;
struct Bars {
    height: Vec<i32>,
}

impl Bars {
    fn new(height: Vec<i32>) -> Self {
        Self { height }
    }

    fn with_water(&self) {
        println!();
        if let Some(top) = self.height.iter().max() {
            let mut h0 = *top;
            while h0 > 0 {
                let mut blocks = Vec::new();

                print!("{}", h0);
                for (i, h) in self.height.iter().enumerate() {
                    if h >= &h0 {
                        blocks.push(i);
                        print!(" ▨");
                    } else {
                        print!("  ");
                    }
                }
                let (block_begin, block_end) = (blocks[0], (blocks[blocks.len() - 1]));
                let volumn = block_end - block_begin + 1 - blocks.len();
                print!(" {}", volumn);
                println!();
                h0 -= 1;
            }
        }
        print!("0");
        for (i, _) in self.height.iter().enumerate() {
            print!("{:2}", i);
        }
        println!();
    }
}

impl Display for Bars {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        if let Some(top) = self.height.iter().max() {
            let mut h0 = *top;
            while h0 > 0 {
                write!(f, "{}", h0)?;
                for h in &self.height {
                    if h >= &h0 {
                        write!(f, " ▨")?;
                    } else {
                        write!(f, "  ")?;
                    }
                }
                writeln!(f)?;
                h0 -= 1;
            }
        }
        Ok(())
    }
}
// @lc code=start
use std::collections::HashMap;

impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let mut vs = 0;
        let mut map = HashMap::new();
        for (i, h) in height.iter().enumerate() {
            if *h > 0 {
                let mut h = *h;
                while h > 0 {
                    let v = map.entry(h).or_insert(Vec::new());
                    v.push(i);
                    h -= 1;
                }
            }
        }
        for m in map {
            // print!("{:?}", m);
            let v = m.1;
            // let (begin, end) = (v[0], v[v.len() - 1]);
            if let (Some(begin), Some(end)) = (v.first(), v.last()) {
                let a = end - begin + 1 - v.len();
                vs += a;
                // println!(" {a}");
            }
        }
        vs as i32
    }
}

// @lc code=end
fn main() {
    let heights = vec![
        vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1],
        vec![4, 2, 0, 3, 2, 5],
    ];

    for h in heights {
        let b = Bars::new(h.clone());
        // println!("{}", b);
        b.with_water();
        println!("total water is {}", Solution::trap(h));
    }

    let v = vec![];

    println!("tatol water is {}", Solution::trap(v));
}
