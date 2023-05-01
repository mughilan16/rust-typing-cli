use rand::Rng;

fn get_random_number() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..8)
}

pub fn get_random_strings() -> [String; 4] {
    let mut numbers: Vec<usize> = vec![];
    for _ in 0..4 {
        numbers.push(get_random_number());
    }
    let string = vec![
        "can need help long some state see who turn then head first ",
        "another become down other life program way head home before",
        "give from another may help each change around about on new ",
        "early play other while feel take with order write what in  ",
        "without right lead possible out word under this have not or",
        "new about consider possible group or a help public because ",
        "order never under open make seem do from into know new own ",
        "can old seem which how increase way of in end another keep ",
        "also any life around with problem any very from course what",
    ];
    let strings: [String; 4] = [
        String::from(string[numbers[0]]),
        String::from(string[numbers[1]]),
        String::from(string[numbers[2]]),
        String::from(string[numbers[3]]),
    ];
    strings
}
