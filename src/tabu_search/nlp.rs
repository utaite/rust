use std::collections::VecDeque;
use rand::Rng;
use rand_distr::num_traits::abs;

type ARR = [i64; MAX];

// 이전 연산에 비해 나아지지 않았을 때 최대 횟수로 시뮬레이션의 종료 조건
const MAX_NOT_IMPROVED: usize = 1000;

// 변수 x의 범위(2^MAX < 100)
const MAX: usize = 7;

// 타부 리스트 크기
const N: usize = 10;

// 함수 f(x)
fn func(arr: ARR) -> i64 {
    let x = to_decimal(arr);
    return (12 * x.pow(5)) - (975 * x.pow(4)) + (28000 * x.pow(3)) - (345000 * x.pow(2)) + (1800000 * x);
}

// 2진수를 10진수로 변환하는 함수
fn to_decimal(arr: ARR) -> i64 {
    (0..MAX)
        .map(|i| arr[i] << (MAX - i - 1))
        .sum()
}

fn main() {
    // 타부 리스트
    let mut tabu_list: VecDeque<i64> = VecDeque::new();

    // 초기 해 생성
    let mut init_v: ARR = [1; MAX];
    while to_decimal(init_v) > 100 {
        for i in 0..MAX {
            init_v[i] = rand::thread_rng().gen_range(0..2);
        }
    }

    // 현재 해
    let mut cur_v: ARR = init_v;

    // 현재 해의 함수값
    let mut zc: i64 = func(cur_v);

    // 최적해
    let mut best_x: ARR = cur_v;

    // 최적해의 함수값
    let mut best_z: i64 = zc;

    // 이전 연산에 비해 나아지지 않았을 때 횟수
    let mut not_improved: usize = 0;

    while not_improved < MAX_NOT_IMPROVED {
        let vec: Vec<(ARR, i64, i64)> = (0..MAX)
            .map(|i| {
                let mut next_v: ARR = cur_v;
                for j in 0..MAX {
                    if i == j {
                        next_v[i] = abs(cur_v[i] - 1);
                    }
                }
                (next_v, to_decimal(next_v), func(next_v))
            })
            .filter(|&(_, number, _)| number <= 100 && !tabu_list.contains(&number))
            .collect();
        let (next_v, _, zn) = vec.into_iter().max_by(|a, c| a.2.cmp(&c.2)).unwrap();

        if best_z < zn {
            best_z = zn;
            best_x = next_v;
            not_improved = 0;
            tabu_list.push_back(zc);
        } else {
            not_improved += 1;
        }

        cur_v = next_v;
        zc = zn;
        if tabu_list.len() > N {
            tabu_list.pop_front();
        }
    }

    println!("최적해: {} {:?}", to_decimal(best_x), best_x);
    println!("최적해의 적합도: {}", best_z);
}
