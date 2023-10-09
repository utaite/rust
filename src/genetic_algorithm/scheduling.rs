use std::cmp::{max, min};
use rand::prelude::SliceRandom;
use rand::Rng;

// 파라미터 초기화

// 타입 선언
type Calc = (Vec<usize>, Vec<usize>, Vec<i64>, i64);

// Upstream conveyor의 수
const CONVEYOR_LENGTH: usize = 3;

// Conveyor당 자동차 수
const CONVEYOR_CAR_LENGTH: usize = 5;

// 모든 Conveyor의 자동차 수
const MAX: usize = CONVEYOR_LENGTH * CONVEYOR_CAR_LENGTH;

// 모집단 크기
const N: usize = 50;

// 이전 연산에 비해 나아지지 않았을 때 최대 횟수로 시뮬레이션의 종료 조건
const MAX_NOT_IMPROVED: usize = 10;

// 교차 연산이 발생할 확률
const RC: f64 = 0.95;

// 돌연변이 연산이 발생할 확률
const RM: f64 = 0.05;

// 교차 연산이 발생했을 때, 일점 교차가 발생할 확률
const RC_ONE_POINT: f64 = 0.5;

// 자동차 준비 비용
static mut ARR: [[i64; MAX]; MAX] = [[0; MAX]; MAX];

// Conveyor의 순서를 통해 (Conveyor의 순서, Conveyor의 순서에 따른 차량의 순서, Conveyor에 진입한 순서대로 발생한 비용의 순서, 비용의 합)을 계산하는 함수
unsafe fn calculation(conveyor_index_vec: Vec<usize>) -> Calc {
    // Conveyor의 등장 횟수 순서
    let mut conveyor_count_vec: Vec<usize> = (0..CONVEYOR_LENGTH).map(|_| 0).collect();
    // Conveyor의 순서에 따른 차량의 순서
    let mut car_index_vec: Vec<usize> = vec![];

    // Conveyor의 현재 위치와 등장 횟수를 통해 차량의 순서를 계산
    for i in 0..conveyor_index_vec.len() {
        // Conveyor의 현재 위치
        let cur_conveyor_index = conveyor_index_vec[i];
        let car_index = CONVEYOR_CAR_LENGTH * cur_conveyor_index + conveyor_count_vec[cur_conveyor_index];
        car_index_vec.push(car_index);

        conveyor_count_vec[cur_conveyor_index] += 1;
    }

    // 초기 모집단 적합도 평가
    // Conveyor에 진입한 순서대로 발생한 비용의 순서
    let value_vec: Vec<i64> = (1..car_index_vec.len())
        .map(|i| ARR[car_index_vec[i - 1]][car_index_vec[i]])
        .collect();
    let value: i64 = value_vec.iter().sum();

    // (Conveyor의 순서, Conveyor의 순서에 따른 차량의 순서, Conveyor에 진입한 순서대로 발생한 비용의 순서, 비용의 합)
    (conveyor_index_vec, car_index_vec, value_vec, value)
}

fn main() {
    unsafe {
        // 자동차 준비 비용을 입력받기 위한 문자열
        let mut buf: String = String::new();

        /* 자동차 준비 비용을 입력받음
        0 8 0 0 0 8 2 5 5 1 7 6 4 5 4
        0 0 3 0 0 8 6 8 4 5 4 6 2 5 8
        0 0 0 6 0 7 0 9 3 1 8 1 2 0 8
        0 0 0 0 0 4 5 3 0 3 6 5 2 10 7
        0 0 0 0 0 8 0 9 1 4 4 6 6 8 1
        8 0 5 7 6 0 6 0 0 0 9 4 7 7 9
        9 9 10 3 6 0 0 10 0 0 6 4 6 2 6
        2 2 1 5 2 0 0 0 10 0 9 3 2 6 9
        6 6 4 9 1 0 0 0 0 5 4 6 0 0 6
        9 10 7 9 4 0 0 0 0 0 10 6 9 4 6
        5 7 4 5 2 2 8 9 10 1 0 4 0 0 0
        1 9 8 6 4 1 4 1 7 7 0 0 7 0 0
        7 2 6 5 7 3 1 8 10 4 0 0 0 2 0
        10 0 2 9 3 9 5 2 5 3 0 0 0 0 8
        7 6 1 1 4 10 4 5 9 8 0 0 0 0 0
        */
        for i in 0..MAX {
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            let vec: Vec<i64> = buf.trim_end().split_whitespace().map(|x| x.parse().unwrap()).collect();
            vec.into_iter().enumerate().for_each(|(j, x)| {
                ARR[i][j] = x;
            });
        }

        // N 크기의 모집단 생성
        let mut vec: Vec<Calc> = (0..N)
            .map(|_| {
                // Conveyor의 순서로 (Upstream conveyor의 수 * Conveyor당 자동차 수)만큼 생성
                let mut conveyor_index_vec: Vec<usize> = (0..CONVEYOR_LENGTH).flat_map(|j|
                    (0..CONVEYOR_CAR_LENGTH)
                        .map(|_| j)
                        .collect::<Vec<usize>>()
                ).collect();
                // Conveyor의 순서를 랜덤하게 셔플
                conveyor_index_vec.shuffle(&mut rand::thread_rng());

                // Conveyor의 순서를 통해 계산
                calculation(conveyor_index_vec)
            })
            .collect();

        // 최적해 순서로 오름차순 정렬
        vec.sort_by(|a, c| a.3.cmp(&c.3));
        vec.iter().enumerate().for_each(|(i, x)| {
            println!("[초기 모집단] {}번째 요소 / 총 비용: {} / 컨베이어의 순서: {:?} / 차량의 순서: {:?} / 비용의 순서: {:?}", i + 1, x.3, x.0, x.1, x.2);
        });

        // 연산 결과
        let mut result: Vec<Calc> = vec![];

        // 연산 횟수
        let mut count = 0;

        // 이전 연산에 비해 나아지지 않은 횟수
        let mut not_improved = 0;

        // 반복 시작
        while not_improved < MAX_NOT_IMPROVED {
            count += 1;
            // 최적해 찾기
            let max_before = vec.first().unwrap().3;

            while vec.len() > 1 {
                // 부모 염색체 생성
                let (first_parent, second_parent) = (vec.pop().unwrap(), vec.pop().unwrap());

                // 자식 염색체 초기화
                let mut first_child: Vec<usize> = vec![];
                let mut second_child: Vec<usize> = vec![];

                // 교차연산
                if rand::thread_rng().gen_range(0.0..1.0) < RC {
                    // 일점교차
                    if rand::thread_rng().gen_range(0.0..1.0) < RC_ONE_POINT {
                        // 염색체의 한 점을 임의로 정하고, 그 점을 중심으로 유전자를 교차함
                        let rand_index = rand::thread_rng().gen_range(1..MAX);
                        println!("교차연산 - 일점교차: {}", rand_index);

                        // 기준점 이전
                        for i in 0..rand_index {
                            first_child.push(first_parent.0[i]);
                            second_child.push(second_parent.0[i]);
                        }

                        // 첫 번째 부모의 기준점 이후부터 끝까지
                        let mut first_child_vec: Vec<usize> = (rand_index..MAX)
                            // 첫 번째 부모의 요소를 가져온 후
                            .map(|i| first_parent.0[i])
                            // 두 번째 부모에서 같은 요소의 순서로 변환
                            .map(|x| second_parent.0.iter().position(|&y| x == y).unwrap())
                            .collect();

                        // 두 번째 부모의 인덱스 순서로 오름차순 정렬 후 첫 번째 자식에게 삽입
                        first_child_vec.sort();
                        first_child_vec.into_iter().for_each(|i| {
                            first_child.push(second_parent.0[i]);
                        });

                        // 두 번째 부모의 기준점 이후부터 끝까지
                        let mut second_child_vec: Vec<usize> = (rand_index..MAX)
                            // 두 번째 부모의 요소를 가져온 후
                            .map(|i| second_parent.0[i])
                            // 첫 번째 부모에서 같은 요소의 순서로 변환
                            .map(|x| first_parent.0.iter().position(|&y| x == y).unwrap())
                            .collect();

                        // 첫 번째 부모의 인덱스 순서로 오름차순 정렬 후 두 번째 자식에게 삽입
                        second_child_vec.sort();
                        second_child_vec.into_iter().for_each(|i| {
                            second_child.push(first_parent.0[i]);
                        });
                    }
                    // 이점 교차
                    else {
                        // 염색체의 두 점을 임의로 정하고, 그 사이에 있는 유전자를 교차함
                        let rand_first_index = rand::thread_rng().gen_range(0..MAX);
                        let mut rand_second_index = rand_first_index;
                        while rand_first_index == rand_second_index {
                            rand_second_index = rand::thread_rng().gen_range(0..MAX);
                        }
                        let rand_start_index = min(rand_first_index, rand_second_index);
                        let rand_end_index = max(rand_first_index, rand_second_index);
                        println!("교차연산 - 이점교차: {}, {}", rand_start_index, rand_end_index);

                        // 첫 번째 기준점 이전
                        for i in 0..rand_start_index {
                            first_child.push(first_parent.0[i]);
                            second_child.push(second_parent.0[i]);
                        }

                        // 첫 번째 부모의 기준점 사이
                        let mut first_child_vec: Vec<usize> = (rand_start_index..rand_end_index)
                            // 첫 번째 부모의 요소를 가져온 후
                            .map(|i| first_parent.0[i])
                            // 두 번째 부모에서 같은 요소의 순서로 변환
                            .map(|x| second_parent.0.iter().position(|&y| x == y).unwrap())
                            .collect();

                        // 두 번째 부모의 인덱스 순서로 오름차순 정렬 후 첫 번째 자식에게 삽입
                        first_child_vec.sort();
                        first_child_vec.into_iter().for_each(|i| {
                            first_child.push(second_parent.0[i]);
                        });

                        // 두 번째 부모의 기준점 사이
                        let mut second_child_vec: Vec<usize> = (rand_start_index..rand_end_index)
                            // 두 번째 부모의 요소를 가져온 후
                            .map(|i| second_parent.0[i])
                            // 첫 번째 부모에서 같은 요소의 순서로 변환
                            .map(|x| first_parent.0.iter().position(|&y| x == y).unwrap())
                            .collect();

                        // 첫 번째 부모의 인덱스 순서로 오름차순 정렬 후 두 번째 자식에게 삽입
                        second_child_vec.sort();
                        second_child_vec.into_iter().for_each(|i| {
                            second_child.push(first_parent.0[i]);
                        });

                        // 두 번째 기준점 이후
                        for i in rand_end_index..MAX {
                            first_child.push(first_parent.0[i]);
                            second_child.push(second_parent.0[i]);
                        }
                    }
                }
                // 교차연산 실패 시
                else {
                    first_child = first_parent.0.clone();
                    second_child = second_parent.0.clone();
                }

                // 돌연변이 연산
                if rand::thread_rng().gen_range(0.0..1.0) < RM {
                    // 유전자 일부가 바뀌는 현상(위치 교환)
                    let rand_first_index = rand::thread_rng().gen_range(0..MAX);
                    let mut rand_second_index = rand_first_index;
                    while rand_first_index == rand_second_index {
                        rand_second_index = rand::thread_rng().gen_range(0..MAX);
                    }
                    let first = (first_child[rand_first_index], first_child[rand_second_index]);
                    let second = (second_child[rand_first_index], second_child[rand_second_index]);
                    println!("돌연변이 연산 - 위치 교환: {}, {}", rand_first_index, rand_second_index);

                    for i in 0..MAX {
                        // 첫 번째 위치인 경우 두 번째 위치로 교환
                        if i == rand_first_index {
                            first_child[i] = first.1;
                            second_child[i] = second.1;
                        }
                        // 두 번째 위치인 경우 첫 번째 위치로 교환
                        else if i == rand_second_index {
                            first_child[i] = first.0;
                            second_child[i] = second.0;
                        }
                    }
                }

                result.push(first_parent);
                result.push(second_parent);
                result.push(calculation(first_child));
                result.push(calculation(second_child));
            }

            // 최적해 순서로 오름차순 정렬 및 선택
            result.sort_by(|a, c| a.3.cmp(&c.3));
            while result.len() > N {
                result.pop();
            }
            while !result.is_empty() {
                vec.push(result.pop().unwrap());
            }
            vec.reverse();

            let max_after = vec.first().unwrap().3;
            not_improved = if max_after > max_before { 0 } else { not_improved + 1 };
        }

        vec.iter().take(10).enumerate().for_each(|(i, x)| {
            println!("[{}번째 연산] {}번째 요소 / 총 비용: {} / 컨베이어의 순서: {:?} / 차량의 순서: {:?} / 비용의 순서: {:?}", count, i + 1, x.3, x.0, x.1, x.2);
        });
    }
}
