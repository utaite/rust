use rand::{Rng, thread_rng};
use rand_distr::{StandardNormal};

// 1회 반복당 연산 횟수
const MAX_REPETITION: usize = 5;

// 온도 스케쥴
const T: [f64; 5] = [0.2, 0.5, 0.5, 0.5, 0.5];

// 최대 범위
const UB: f64 = 100.0;

// 최소 범위
const LB: f64 = 0.0;

// 함수 f(x)
fn func(x: f64) -> f64 {
    return (12.0 * x.powi(5)) - (975.0 * x.powi(4)) + (28000.0 * x.powi(3)) - (345000.0 * x.powi(2)) + (1800000.0 * x);
}

fn main() {
    // 표준편차
    let sigma: f64 = (UB - LB) / 6.0;

    // 초기 해 생성
    let init_v: f64 = (UB - LB) / 2.0;

    // 현재 해
    let mut cur_v: f64 = init_v;

    // 현재 해의 함수값
    let mut zc = func(cur_v);

    // 최적해
    let mut best_x = cur_v;

    // 최적해의 함수값
    let mut best_z = zc;

    // 반복 횟수
    let mut repetition: usize = 0;

    // 연산 횟수
    let mut count: usize = 0;

    // 현재 온도 인덱스
    let mut cur_t_index: usize = 0;

    // 현재 온도
    let mut cur_t: f64 = func(cur_v) * T[cur_t_index];

    // (1회 반복당 연산 횟수 * 온도 스케쥴의 길이)만큼 반복
    while count < MAX_REPETITION * T.len() {
        repetition += 1;
        count += 1;

        loop {
            // 평균이 0이고 표준편차가 1인 정규분포를 따르는 난수 생성
            let rand_v: f64 = thread_rng().sample(StandardNormal);
            // 다음 해 후보 생성
            let next_v = cur_v + rand_v * sigma;

            // 가능 해인지 확인
            if next_v >= LB && next_v <= UB {
                // 다음 해 후보의 함수값 생성
                let zn = func(next_v);

                // 다음 해 후보가 현재 해보다 함수값이 클 때
                if zc < zn {
                    zc = zn;
                    cur_v = next_v;

                    // 다음 해 후보가 최적해보다 함수값이 클 때
                    if zn > best_z {
                        best_z = zn;
                        best_x = next_v;
                        break;
                    }
                } else {
                    // e^((현재 해의 함수값 - 다음 해의 함수값) / 현재 온도)
                    let acceptance = ((zc - zn) / cur_t).exp();

                    // 0부터 1 사이의 난수 < acceptance 라면 채택
                    if thread_rng().gen_range(0.0..1.0) < acceptance {
                        zc = zn;
                        cur_v = next_v;
                        break;
                    }
                }
            }
        }

        // 1회 반복당 연산 횟수를 채웠다면 다음 온도를 적용
        if repetition == MAX_REPETITION {
            repetition = 0;
            cur_t_index += 1;

            if cur_t_index < T.len() {
                cur_t *= T[cur_t_index]
            }
        }
    }

    println!("최적해: {}", best_x);
    println!("최적해의 함수값: {}", best_z);
}
