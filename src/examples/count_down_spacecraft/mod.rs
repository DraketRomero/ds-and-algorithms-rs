pub fn count_down_spacecraft(time_to_take_off: i32) -> i32 {
    if time_to_take_off == 0 {
        return 0;
    }

    println!("{}", time_to_take_off);

    return count_down_spacecraft(time_to_take_off - 1);
}
