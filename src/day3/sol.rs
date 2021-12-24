use crate::utils;

#[derive(Debug, Clone)]
struct Diagnostic {
    items: Vec<bool>
}

#[derive(Debug, Clone)]
struct Counts {
    items: Vec<i32>
}

impl Counts {
    fn add_count(&mut self, i: usize) {
        if self.items.len() <= i {
            for _ in 0..(i - self.items.len() + 1) {
                self.items.push(0);
            }
        }
        self.items[i] += 1;
    }
}

fn split_line(s: &str) -> Diagnostic {
    let chars : Vec<_> = s.chars().collect();
    if chars.len() < 2 {
        panic!("Didn't split example well: {:?}", &chars);
    }

    let mut items: Vec<bool> = Vec::new();
    for c in chars {
        match c {
            '0' => items.push(true),
            '1' => items.push(false),
            _ => panic!("Unexpected value: {}", c),
        }
    }
    Diagnostic{items }

}

pub fn a() -> i32 {
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let bools : Vec<Diagnostic> = str_lines.map(split_line).collect();
    let mut counts = Counts{items: Vec::new()};

    for d in &bools {
        for (i, &v) in d.items.iter().enumerate() {
            if v == true {
                counts.add_count(i);
            }
        }
    }
    let mut most: Vec<String> = Vec::new();
    let mut least: Vec<String> = Vec::new();
    let half = &bools.len() / 2;
    for c in counts.items {
        if c > half as i32 {
            most.push((&"1").parse().unwrap());
            least.push((&"0").parse().unwrap());
        } else {
            most.push((&"0").parse().unwrap());
            least.push((&"1").parse().unwrap());
        }
    }
    let most_int = isize::from_str_radix(&*most.concat(), 2).unwrap();
    let least_int = isize::from_str_radix(&*least.concat(), 2).unwrap();

    (most_int * least_int) as i32
}

pub fn b() -> i32 {
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let bools : Vec<Diagnostic> = str_lines.map(split_line).collect();
    let mut counts = Counts{items: Vec::new()};

    for d in &bools {
        for (i, &v) in d.items.iter().enumerate() {
            if v == true {
                counts.add_count(i);
            }
        }
    }

    let mut most_num : Option<i32> = None;
    let mut least_num : Option<i32> = None;

    let mut most: Vec<Diagnostic> = bools.to_vec();
    let mut least: Vec<Diagnostic> = most.to_vec();

    let half = (&bools.len() + 1) / 2;
    for (i, &c) in counts.items.iter().enumerate() {
        let ones_common = c >= (half as i32);
        match most_num {
            None => most = most.clone().iter().filter(|x| x.items[i] == ones_common).collect(),
            _ => (),
        }
        match least_num {
            None => least = least.iter().filter(|x| x.items[i] != ones_common).collect(),
            _ => (),
        }

        if most.len() == 0 {
            panic!("{}", most.len());
        } else if least.len() == 0 {
            panic!("{}", least.len());
        }

        if most.len() == 1 {
            let mut ms = String::new();
            for &c in &most[0].items {
                if c {
                    ms.push('1');
                } else {
                    ms.push('0');
                }
            }
            most_num = Some(isize::from_str_radix(&*ms, 2).unwrap() as i32);
        }
        if least.len() == 1 {
            let mut ls = String::new();
            for &c in &least[0].items {
                if c {
                    ls.push('1');
                } else {
                    ls.push('0');
                }
            }
            least_num = Some(isize::from_str_radix(&*ls, 2).unwrap() as i32);
        }

    }

    (most_num.unwrap() * least_num.unwrap()) as i32
}
