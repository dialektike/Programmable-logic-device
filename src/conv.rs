//! # 변환(conv) 모듈
//!
//! 이 모듈에는 비트와 숫자를 변환하는 함수가 들어 있습니다. 이 모듈 안에 들어 있는 코드는
//! 기본적으로 편의상 최적화를 실행하지 못했습니다. 기능을 구현하는데 급급한 면이 있습니다. 
//! 참고하세요.
//!
//! ## 목록
//! 
//! - `from_eight_bool_to_eight_bit`: 8 자리 `bool` Array 을 8 자리 bit Array 으로 변환
//! - `from_eight_bit_to_eight_bool`: 8 자리 bit Array 를 8 자리 `bool` Array 으로 변환
//! - `from_eight_bit_to_one_u8_int`: 8 자리 bit Array를 1개의 `u8` int로 변환

use crate::adder::eight_bit_adder;

///  `bool`로 된 8개의 array를 이진수로 된 8 bit array로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a = [false, true, true, false, true, false, false, true];
///     let test = eight_bool_to_bit(temp_a);
/// ```
pub fn from_eight_bool_to_eight_bit(input: [bool; 8]) -> [u8; 8] {
    // 즉 0이 8개라는 의미
    let mut temp_result: [u8; 8] = [0; 8];
    for (temp_num, i) in input.into_iter().enumerate().rev() {
        if i {
            temp_result[temp_num] = i as u8;
        }
    }
    temp_result
}
///  `bool`로 된 8 자리 array 를 이진수로 된 8 bit array로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a = [false, true, true, false, true, false, false, true];
///     let test = from_eight_bit_to_eight_bool(temp_a);
/// ```
pub fn from_eight_bit_to_eight_bool(input: [i32; 8]) -> [bool; 8] {
    //  즉 `false`가 8개라는 이야기
    let mut temp_result: [bool; 8] = [false; 8];

    for (temp_num, i) in input.into_iter().enumerate() {
        if i == 0b1 {
            temp_result[temp_num] = true;
        }
    }
    temp_result
}

///  `u8`로 된 이진수 8 자리 리스트를 `u8`로 된 `int`로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a = [0, 1, 1, 0, 1, 0, 0, 1];
///     let test = from_eight_bit_to_one_u8_int(temp_a);
/// ```
pub fn from_eight_bit_to_one_u8_int(input: [u8; 8]) -> u8 {
    let mut temp_result = 0b0;
    let mut temp_index = 0;
    for i in input.into_iter().rev() {
        if i == 1 {
            // println!("index:{}", index);
            // println!("i:{:0b}", u32::pow(0b10, index));
            // 거듭제곱
            let temp = u8::pow(0b10, temp_index);
            temp_result += temp;
        }
        temp_index += 1;
    }
    println!("이진수: {:0b}", temp_result);
    temp_result
}

///  `u8` 크기인 `int`를  `bool`로 된 8 자리 리스트로 변경하는 함수
///
/// 다음과 같이 작동합니다.
///  ```rust
///     let temp_a: [u8; 8] = [0, 0, 0, 0, 0, 1, 0, 1];
///     let result = from_eight_bit_to_one_u8_int(temp_a);
///     assert_eq!(result, 5);
/// ```
pub fn u8_int_to_eight_bit(input: u8) -> [bool; 8] {
    let mut temp_result = [false; 8];
    let add_one = [false, false, false, false, false, false, false, true];
    for _n in 0..input {
        temp_result = eight_bit_adder(temp_result, add_one, false, true).0;
    }
    temp_result
}
