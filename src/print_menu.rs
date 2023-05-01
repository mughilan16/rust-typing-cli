use std::time::Duration;
pub enum Result {
    Type,
    Correct,
    Wrong,
}
pub fn print_menu() {
    println!("╭────────────────────────────────────────────────────────────╮");
    println!("│                          Menu                              │");
    println!("│────────────────────────────────────────────────────────────│");
    println!("│                                                            │");
    println!("│ 1. Random Sentence                                         │");
    println!("│ 2. Pratice Mistake words                                   │");
    println!("│ 3. Words                                                   │");
    println!("│ 4. Exit                                                    │");
    println!("╰────────────────────────────────────────────────────────────╯");
}

fn arrange_time(time: f32) -> String {
    let time: i32 = time as i32;
    let time_string: String;
    if time < 10 {
        time_string = time.to_string() + " sec  ";
    } else if time < 100 {
        time_string = time.to_string() + " sec ";
    } else {
        time_string = time.to_string() + " sec";
    }
    time_string
}

pub fn print_string(
    string: String,
    string_id: usize,
    total: usize,
    result: Result,
    time: Duration,
) {
    let result = match result {
        Result::Correct => String::from("Correct"),
        Result::Wrong => String::from("Wrong  "),
        Result::Type => String::from("Type   "),
    };
    let time = arrange_time(time.as_secs_f32());

    println!("{} {} {}", string, string_id, total);
    println!("╭─────────────────────────────────────────────────────────────╮╭─────────╮╭───────╮╭─────────╮╭─────────────╮");
    println!("│                       Type the below word                   ││ Result  ││ Line  ││  Time   ││  Speed(wpm) │");
    println!("│─────────────────────────────────────────────────────────────││─────────││───────││─────────││─────────────│");
    println!("│{string}  ││ {result} ││ {string_id}/{total}   ││ {time} ││10  wpm      │");
    println!("╰─────────────────────────────────────────────────────────────╯╰─────────╯╰───────╯╰─────────╯╰─────────────╯");
}
