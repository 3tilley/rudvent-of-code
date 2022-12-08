use crate::DayData;

struct Elf {
    items: Vec<u64>,
}

impl Elf {
    fn new(items: Vec<u64>) -> Self {
        Self { items }
    }

    fn calorie_sum(&self) -> u64 {
        self.items.iter().sum()
    }
}

pub(crate) fn solution(example: bool) -> u64 {
    let day_data = DayData::new(1, false);
    let text = if example {
        day_data.example_1()
    } else {
        day_data.input_1()
    };
    let mut elves = Vec::new();
    let mut cals = Vec::new();
    for line in text.lines() {
        println!("{:#?}", &line.bytes());
        println!("{:#?}", &line.chars());
        if line.len() == 0 {
            elves.push(Elf::new(cals));
            cals = Vec::new();
        } else {
            cals.push(line.parse::<u64>().unwrap());
        }
    }
    if cals.len() > 0 {
        elves.push(Elf::new(cals));
    }
    elves.iter().map(|elf| elf.calorie_sum()).max().unwrap()
}