const START: i32 = 264793;
const END: i32 = 803935;

fn is_valid_passcode(code: i32) -> bool {
    // Must be within input range
    if code < START || code > END {
        return false
    }

    // Two adjacent digits are the same
    let s = code.to_string();
    let mut has_adjacent = false;
    for i in 0..s.len()-1 {
        if s.as_bytes()[i] == s.as_bytes()[i+1] {
            has_adjacent = true;
        }

        // From right to left, digits never decrease
        if s.as_bytes()[i] > s.as_bytes()[i+1] { return false }
    }

    return has_adjacent
}

fn has_solo_double(code: i32) -> bool {
    let s = code.to_string();
    let b = s.as_bytes();

    for i in 0..s.len()-1 {
        if b[i] == b[i+1] &&
           (i == 0 || b[i] != b[i-1]) &&
           (i == s.len()-2 || b[i] != b[i+2]) {
               return true
        }
    }

    return false
}

fn part_one() {
    let total: i32 = (START..END).map(|i| is_valid_passcode(i) as i32).sum();
    println!("{}", total);
}

fn part_two() {
    let total: i32 = (START..END)
        .map(|i| is_valid_passcode(i) && has_solo_double(i))
        .map(|b| b as i32)
        .sum();

    println!("{}", total)
}

fn main() {
    part_one();
    part_two();
}
