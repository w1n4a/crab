pub fn reverse(janedoy: &String) -> String {
    let mut vec: Vec<char> = Vec::new();
    let mut res: String = String::new();

    for i in janedoy.chars() {
        vec.push(i);
    }

    for i in 1..vec.len() + 1 {
        res.push(vec[vec.len() - i]);
    }

    res
}

pub fn line_count(janedoy: &String) -> usize {
    let mut res: usize = 1;

    for i in janedoy.chars() {
        if i == '\n' {
            res += 1;
        }
    }

    res
}

pub fn len(janedoy: &String) -> usize {
    let mut res: usize = 0;

    for _ in janedoy.chars() {
        res += 1;
    }

    res
}
