use rand::Rng;

// 변수 x의 범위(2^MAX < 100)
const MAX: usize = 7;

// 모집단 크기
const N: usize = 10;

// 교차 연산이 발생할 확률
const RC: f64 = 0.95;

// 돌연변이 연산이 발생할 확률
const RM: f64 = 0.05;

// 함수 f(x)
fn func(arr: [usize; MAX]) -> i64 {
    let x = to_decimal(arr) as i64;
    return (12 * x.pow(5)) - (975 * x.pow(4)) + (28000 * x.pow(3)) - (345000 * x.pow(2)) + (1800000 * x);
}

// 2진수를 10진수로 변환하는 함수
fn to_decimal(arr: [usize; MAX]) -> usize {
    (0..MAX)
        .map(|i| arr[i] << (MAX - i - 1))
        .sum()
}

fn main() {
    // N 크기의 모집단 생성
    let mut vec: Vec<([usize; MAX], usize, i64)> = (0..N)
        .map(|_| {
            let mut arr: [usize; MAX] = [1; MAX];
            while to_decimal(arr) > 100 {
                for i in 0..MAX {
                    arr[i] = rand::thread_rng().gen_range(0..2);
                }
            }
            (arr, to_decimal(arr), func(arr))
        })
        .collect();

    // 연산 결과
    let mut result: Vec<([usize; MAX], usize, i64)> = vec![];

    // 연산 횟수
    let mut count = 0;

    // 이전 연산에 비해 나아지지 않았을 때 횟수
    let mut not_improved = 0;

    // 최적해 순서로 정렬
    vec.sort_by(|&a, &c| c.2.cmp(&a.2));
    vec.iter().enumerate().for_each(|(i, x)| {
        println!("초기 모집단 / {}번째 요소: {:?}", i + 1, x);
    });

    while not_improved < 10 {
        count += 1;
        let max_before = vec.first().unwrap().2;

        while vec.len() > 1 {
            let (first_parent, second_parent) = (vec.pop().unwrap(), vec.pop().unwrap());
            let mut first_child: [usize; MAX] = [1; MAX];
            let mut first_flag = false;
            let mut second_child: [usize; MAX] = [1; MAX];
            let mut second_flag = false;

            for i in 0..MAX {
                first_child[i] = first_parent.0[i];
                second_child[i] = second_parent.0[i];
            }

            // 교차연산
            if rand::thread_rng().gen_range(0.0..1.0) < RC {
                while to_decimal(first_child) > 100 || !first_flag {
                    for i in 0..MAX {
                        // 부모의 특성 반영
                        first_child[i] = if first_parent.0[i] == second_parent.0[i] { first_parent.0[i] } else { rand::thread_rng().gen_range(0..2) };
                    }
                    first_flag = true;
                }

                while to_decimal(second_child) > 100 || !second_flag {
                    for i in 0..MAX {
                        // 부모의 특성 반영
                        second_child[i] = if first_parent.0[i] == second_parent.0[i] { first_parent.0[i] } else { rand::thread_rng().gen_range(0..2) };
                    }
                    second_flag = true;
                }
            }

            // 돌연변이 연산
            if rand::thread_rng().gen_range(0.0..1.0) < RM {
                let rand = rand::thread_rng().gen_range(0.0..1.0);
                let mut mutation_index = 0;

                for i in 0..MAX {
                    if ((rand * MAX as f64) as usize) < i {
                        mutation_index = i;
                        break;
                    }
                }

                first_child[mutation_index] = if first_child[mutation_index] == 0 { 1 } else { 0 };

                if to_decimal(first_child) > 100 {
                    first_child[mutation_index] = if first_child[mutation_index] == 0 { 1 } else { 0 };
                }
            }

            result.push(first_parent);
            result.push(second_parent);
            result.push((first_child, to_decimal(first_child), func(first_child)));
            result.push((second_child, to_decimal(second_child), func(second_child)));
        }

        // 최적해 순서로 정렬 및 선택
        result.sort_by(|&a, &c| c.2.cmp(&a.2));
        while result.len() > 10 {
            result.pop();
        }
        while !result.is_empty() {
            vec.push(result.pop().unwrap());
        }
        vec.reverse();

        let max_after = vec.first().unwrap().2;
        not_improved = if max_after > max_before { 0 } else { not_improved + 1 };

        println!();
        vec.iter().enumerate().for_each(|(i, x)| {
            println!("{}번째 연산 / {}번째 요소: {:?}", count, i + 1, x);
        });
    }
}
