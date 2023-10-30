use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;

type ARR = [usize; MAX];

// 1회 반복당 연산 횟수
const MAX_REPETITION: usize = 10000;

// 온도 스케쥴
const T: [f64; 5] = [0.2, 0.5, 0.5, 0.5, 0.5];

// 작업 횟수
const MAX: usize = 20;

// 작업에 대한 처리시간
const COSTS: ARR = [13, 15, 10, 21, 34, 9, 25, 23, 11, 8, 12, 17, 30, 14, 45, 27, 31, 20, 6, 5];

// 함수 f(x)
fn func(arr: ARR) -> i64 {
    return arr.into_iter().map(|i| COSTS[i]).enumerate().fold(0, |a, (i, c)| a + (MAX - i) * c) as i64;
}

fn main() {
    // 초기 해 생성
    let init_v: ARR = core::array::from_fn(|i| i);

    // 현재 해
    let mut cur_v: ARR = init_v;

    // 현재 해의 적합도
    let mut zc: i64 = func(cur_v);

    // 최적해
    let mut best_x: ARR = cur_v;

    // 최적해의 적합도
    let mut best_z: i64 = zc;

    // 반복 횟수
    let mut repetition: usize = 0;

    // 연산 횟수
    let mut count: usize = 0;

    // 현재 온도 인덱스
    let mut cur_t_index: usize = 0;

    // 현재 온도
    let mut cur_t: f64 = func(cur_v) as f64 * T[cur_t_index];

    // (1회 반복당 연산 횟수 * 온도 스케쥴의 길이)만큼 연산 반복
    while count < MAX_REPETITION * T.len() {
        repetition += 1;
        count += 1;

        loop {
            // 다음 해 후보 생성
            let mut next_v = core::array::from_fn(|i| i);
            next_v.shuffle(&mut thread_rng());

            // 다음 해 후보의 적합도 생성
            let zn = func(next_v);

            // 다음 해 후보가 현재 해보다 적합도가 작을 때
            if zn < zc {
                zc = zn;
                cur_v = next_v;

                // 다음 해 후보가 최적해보다 적합도가 작을 때
                if zn < best_z {
                    best_z = zn;
                    best_x = next_v;
                    break;
                }
            } else {
                // 채택 확률 계산 e^((현재 해의 적합도 - 다음 해의 적합도) / 현재 온도)
                let acceptance = ((zc - zn) as f64 / cur_t).exp();

                // 0부터 1 사이의 난수 < acceptance 라면 채택
                if thread_rng().gen_range(0.0..1.0) < acceptance {
                    zc = zn;
                    cur_v = next_v;
                    break;
                }
            }
        }

        // 1회 반복당 연산 횟수를 채웠을 때 다음 온도 적용
        if repetition == MAX_REPETITION {
            repetition = 0;
            cur_t_index += 1;

            if cur_t_index < T.len() {
                cur_t *= T[cur_t_index]
            }
        }
    }

    println!("최적해: {:?}", best_x);
    println!("최적해의 적합도: {}", best_z);
}
