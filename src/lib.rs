/// 렐레이 구현 함수
///
/// 전압이 들어가면서 입력이 들어오면 전기가 나간다.
/// 전압이 들어가지만 입력이 안 들어오면 전기가 안 나간다.
///
/// Parameters
///
/// - v: 전원
/// - input: 릴레이를 실행
///
pub fn relay(input: bool, v: bool) -> bool {
    v & input
}

/// `AND` 게이트 구현 함수
///
/// 두 릴레이를 직렬로 연결한 것입니다.
/// 전압이 들어 오면서 두 입력이 들어올 때만 전기가 나간다.
/// 전압이 들어오지 않으면 당연히 전기가 안 나간다.
///
/// Parameters
///
/// - v: 전원
/// - input_1: 입력 1
/// - input_2: 입력 2
///
pub fn and_gate(input_1: bool, input_2: bool, v: bool) -> bool {
    relay(input_2, relay(input_1, v))
}

/// `OR` 게이트 구현 함수
///
/// 두 릴레이를 병렬로 연결한 것을 표현합니다.
/// 전압이 들어 오면서 두 입력 중 하나 이상이 들어올 때만 전기가 나갑니다.
/// 전압이 들어오지 않으면 당연히 전기가 안 나갑니다.
/// 두 릴레이의 출력 두 개를 전선으로 연결하는 것을 `|` 연산자로 해결했습니다.
///
/// Parameters
///
/// - v: 전원
/// - input_1: 입력 1
/// - input_2: 입력 2
///
pub fn or_gate(v_1: bool, v_2: bool, v: bool) -> bool {
    relay(v_1, v) | relay(v_2, v)
}

/// `NOR` 게이트 구현 함수
///
/// `OR`에서 발생하는 것과 정확하게 반대입니다.
/// 전압이 들어 오면서 두 입력 중 하나 이상이 들어오면 전기가 안 나갑니다.
/// 전압이 들어 오면서 두 입력이 하나도 안 들어와야 전기가 나갑니다.
///
/// Parameters
///
/// - v: 전원
/// - input_1: 입력 1
/// - input_2: 입력 2
///
pub fn nor_gate(v_1: bool, v_2: bool, v: bool) -> bool {
    inverter(v_2, inverter(v_1, v))
}

/// `XOR` 게이트 구현 함수
///
/// 전압이 들어 오면서 두 입력 중 하나 이상이 들어오면 전기가 나갑니다.
/// 그러나 전압이 들어 오면서 두 입력 모두 들어오면 전기가 안 나갑니다.
/// 그래서 배타적(Exclusive) OR 라고 합니다.
///
/// Parameters
///
/// - v: 전원
/// - input_1: 입력 1
/// - input_2: 입력 2
///
pub fn xor_gate(v_1: bool, v_2: bool, v: bool) -> bool {
    and_gate(or_gate(v_1, v_2, v), nand_gate(v_1, v_2, v), v)
}

/// `NAND` 게이트 구현 함수
///
/// `AND`에서 발생하는 것과 정확하게 반대입니다.
/// 전압이 들어 오면서 두 입력 모두 들어올 때만 전기가 나갑니다.
///
/// Parameters
///
/// - v: 전원
/// - input_1: 입력 1
/// - input_2: 입력 2
///
pub fn nand_gate(v_1: bool, v_2: bool, v: bool) -> bool {
    inverter(v_1, v) | inverter(v_2, v)
}

/// `inverter` 구현 함수
///
/// 전압이 들어가면서 입력이 들어오면 전기가 안 나가갑니다.
/// 전압이 들어가면서 입력이 안 들어오면 전기가 나갑니다.
/// 전압이 안 들어가면서 무조건 전기가 안 나갑니다.
///
/// Parameters
///
/// - v: 전원
/// - input: 입력
///
pub fn inverter(input: bool, v: bool) -> bool {
    if v {
        !input
    } else {
        false
    }
}
