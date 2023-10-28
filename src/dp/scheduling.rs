use std::cmp::min;

const MAX: usize = 16;
const LIMIT: i64 = 1000000000;
static mut ARR: [[i64; MAX]; MAX] = [[0; MAX]; MAX];
static mut DP: [[i64; 1 << MAX]; MAX] = [[-1; 1 << MAX]; MAX];

fn main() {
    unsafe {
        let mut buf: String = String::new();
        let n: usize = 15;

        for i in 0..n {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let vec: Vec<i64> = buf.trim_end().split_whitespace().filter(|x| !x.is_empty()).map(|x| x.parse().unwrap()).collect();
            vec.into_iter().enumerate().for_each(|(j, x)| {
                ARR[i][j] = x;
            });
        }

        DP = [[-1; 1 << MAX]; MAX];
        println!("{:b}", 1);
        println!("{}", dfs(0, 1, n));

        DP = [[-1; 1 << MAX]; MAX];
        println!("{:b}", 1 << 5);
        println!("{}", dfs(5, 1 << 5, n));

        DP = [[-1; 1 << MAX]; MAX];
        println!("{:b}", 1 << 10);
        println!("{}", dfs(10, 1 << 10, n));
    }
}

unsafe fn dfs(cur: usize, bit: usize, n: usize) -> i64 {
    if bit == (1 << n) - 1 {
        0
    } else if DP[cur][bit] != -1 {
        DP[cur][bit]
    } else {
        for i in 0..n {
            if ARR[cur][i] >= 0 && (bit & (1 << i)) != 1 << i && ((i == 0 || i == 5 || i == 10) || (bit & (1 << (i - 1))) == 1 << (i - 1)) {
                DP[cur][bit] = if DP[cur][bit] == -1 {
                    ARR[cur][i] + dfs(i, bit | 1 << i, n)
                } else {
                    min(DP[cur][bit], ARR[cur][i] + dfs(i, bit | 1 << i, n))
                };
            }
        }

        if DP[cur][bit] == -1 {
            LIMIT
        } else {
            DP[cur][bit]
        }
    }
}
