// util & function
use regex::Regex;
/*
String값 온도 데이터를 배열로 변환
*/
pub fn string_to_arr(input_string: &String) -> Vec<String> {
    // 4씩 짤라넣기. Request측에서 어떻게 보낼지는 모르겠지만 일단 잘라넣어.
    let mut hex_arr: Vec<String> = Vec::new();
    for chunk in input_string.chars().collect::<Vec<char>>().chunks(4) {
        let chunk_str: String = chunk.iter().collect();
        hex_arr.push(chunk_str);
    }

    hex_arr
}

/*
배열에 있는 hexa값을 decimal로 변환
*/
pub fn hex_to_decimal(hex_strings: Vec<String>) -> Vec<i32> {
    let mut result = Vec::new();

    for hex_str in hex_strings {
        //from_str_radix는 -부호를 추가해서 사용하는방식이므로 그대로 쓸수 없다.
        // 직접 연산을 할수밖에 없다.... sign을 직접 확인하고 -면....
        // i16의 범위(65536)을 빼주고 부호를 다시 적용.
        let mut num = i32::from_str_radix(&hex_str, 16).unwrap();

        if is_minus(&hex_str) {
            num = (65536 - num) * -1;
        }

        result.push(num);
    }

    result
}

/*
부호확인
*/
fn is_minus(value: &str) -> bool {
    let regex = Regex::new(r"^[a-fA-F890]").unwrap();
    regex.is_match(&value[..1])
}
