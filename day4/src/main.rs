fn valid_password_pt1(password: &[u8]) -> bool {
    let mut has_twins = false;
    for i in 0..=4 {
        if password[i+1] < password[i] {
            return false // decreasing digits
        }

        if password[i+1] == password[i] {
            has_twins = true;
        }
    }


    has_twins
}

fn valid_password_pt2(password: &[u8]) -> bool {
    let mut has_twins = false;
    let mut current_group_size = 1;
    for i in 1..=5 {
        if password[i] < password[i - 1] {
            return false // decreasing digits
        }

        if password[i] == password[i - 1] {
            current_group_size += 1;
        } else {
            has_twins = has_twins || current_group_size == 2;
            current_group_size = 1;
        }
    }
    has_twins = has_twins || current_group_size == 2;

    has_twins
}

fn count_valid_password(min: u32, max: u32, valid: fn(&[u8]) -> bool) -> u32 {
    let mut count = 0;

    for attempt in min..=max {
        if valid(attempt.to_string().as_bytes()) {
            count += 1
        }
    }

    count
}

fn main() {
    // input is "307237-769058"
    let (min, max) = (307237, 769058);
    println!("Part 1: valid password count = {}", count_valid_password(min, max, valid_password_pt1));
    println!("Part 2: valid password count = {}", count_valid_password(min, max, valid_password_pt2));
}
