use programmable_logic_device::{
    and_gate, inverter, nand_gate, nor_gate, or_gate, relay, xor_gate,
};

#[cfg(test)]
#[test]
fn relay_works() {
    let result = relay(true, true);
    assert_eq!(result, true);
    let result = relay(true, false);
    assert_eq!(result, false);
    let result = relay(false, true);
    assert_eq!(result, false);
    let result = relay(false, false);
    assert_eq!(result, false);
}

#[test]
fn relay_cascaded_works() {
    // 전력은 들어오고 입력이 있으면 당연히 `true`
    let temp_input = true;
    let temp_v = true;
    let temp_relay = relay(temp_input, temp_v);
    let cascaded_relay = relay(temp_relay, temp_v);
    assert_eq!(cascaded_relay, true);
    // 전력은 들어오는데 입력이 없으면 당연히 `false`
    let temp_input = false;
    let temp_v = true;
    let temp_relay = relay(temp_input, temp_v);
    let cascaded_relay = relay(temp_relay, temp_v);
    assert_eq!(cascaded_relay, false);
}

#[test]
fn relay_series_works() {
    // relay 둘 다 전력이 들어오고
    // 입력도 둘 다 있으면 `true`
    let temp_input_1 = true;
    let temp_input_2 = true;
    let temp_v = true;
    let temp_1_relay = relay(temp_input_1, temp_v);
    let temp_2_relay = relay(temp_input_2, temp_1_relay);
    assert_eq!(temp_2_relay, true);
    // 전력은 들어오는데 입력이 하나만 있으면 출력은 모두 `false`
    let temp_input_1 = true;
    let temp_input_2 = false;
    let temp_v = true;
    let temp_1_relay = relay(temp_input_1, temp_v);
    let temp_2_relay = relay(temp_input_2, temp_1_relay);
    assert_eq!(temp_2_relay, false);
    let temp_input_1 = false;
    let temp_input_2 = true;
    let temp_v = true;
    let temp_1_relay = relay(temp_input_1, temp_v);
    let temp_2_relay = relay(temp_input_2, temp_1_relay);
    assert_eq!(temp_2_relay, false);
}

#[test]
fn and_gate_works() {
    let temp_v = true;
    let result = and_gate(true, true, temp_v);
    assert_eq!(result, true);
    let result = and_gate(true, false, temp_v);
    assert_eq!(result, false);
    let result = and_gate(false, true, temp_v);
    assert_eq!(result, false);
    let result = and_gate(false, false, temp_v);
    assert_eq!(result, false);
    let temp_v: bool = false;
    // 전력이 안 들어오면 당연히 작동하지 않습니다.
    let result = and_gate(true, true, temp_v);
    assert_eq!(result, false);
}

#[test]
fn nand_gate_works() {
    let temp_v = true;
    let result = nand_gate(true, true, temp_v);
    assert_eq!(result, false);
    let result = nand_gate(true, false, temp_v);
    assert_eq!(result, true);
    let result = nand_gate(false, true, temp_v);
    assert_eq!(result, true);
    let result = nand_gate(false, false, temp_v);
    assert_eq!(result, true);
    let temp_v: bool = false;
    // 전력이 안 들어오면 당연히 작동하지 않습니다.
    let result = nand_gate(true, true, temp_v);
    assert_eq!(result, false);
}

#[test]
fn or_gate_works() {
    let temp_v = true;
    let result = or_gate(true, true, temp_v);
    assert_eq!(result, true);
    let result = or_gate(true, false, temp_v);
    assert_eq!(result, true);
    let result = or_gate(false, true, temp_v);
    assert_eq!(result, true);
    let result = or_gate(false, false, temp_v);
    assert_eq!(result, false);
    let temp_v: bool = false;
    // 전력이 안 들어오면 당연히 작동하지 않습니다.
    let result = or_gate(true, true, temp_v);
    assert_eq!(result, false);
}

#[test]
fn xor_gate_works() {
    let temp_v = true;
    // 모두 참이면 거짓, 나머지는 or_gate 와 동일
    let result = xor_gate(true, true, temp_v);
    assert_eq!(result, false);
    let result = xor_gate(true, false, temp_v);
    assert_eq!(result, true);
    let result = xor_gate(false, true, temp_v);
    assert_eq!(result, true);
    let result = xor_gate(false, false, temp_v);
    assert_eq!(result, false);
    let temp_v: bool = false;
    // 전력이 안 들어오면 당연히 작동하지 않습니다.
    let result = or_gate(true, true, temp_v);
    assert_eq!(result, false);
}

#[test]
fn nor_gate_works() {
    let temp_v = true;
    let result = nor_gate(true, true, temp_v);
    assert_eq!(result, false);
    let result = nor_gate(true, false, temp_v);
    assert_eq!(result, false);
    let result = nor_gate(false, true, temp_v);
    assert_eq!(result, false);
    let result = nor_gate(false, false, temp_v);
    assert_eq!(result, true);
    let temp_v: bool = false;
    // 전력이 안 들어오면 당연히 작동하지 않습니다.
    let result = or_gate(false, false, temp_v);
    assert_eq!(result, false);
}

#[test]
fn inverter_works() {
    let result = inverter(true, true);
    assert_eq!(result, false);
    let result = inverter(false, true);
    assert_eq!(result, true);
    let result = inverter(true, false);
    assert_eq!(result, false);
    let result = inverter(false, false);
    assert_eq!(result, false);
}
