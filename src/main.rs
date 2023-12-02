use regex::Regex;
use std::{error, fs, ops::Deref};

fn main() {
    println!("Hello, world!");
    let contents = fs::read_to_string("./inputs.txt");

    let mut text_lines: Vec<String>;
    match contents {
        Ok(con) => {
            text_lines = con
                .lines()
                .map(|x| format!("{}\n", x))
                .collect::<Vec<String>>();
        }
        Err(_) => todo!(),
    }
    let mut answer: u32 = 0;
    let mut nums: Vec<Vec<u32>> = Vec::new();
    for i in text_lines.iter_mut() {
        let mut re: String = i.deref().to_string();
        re = re.replace("zero", "zero0zero");

        re = re.replace("one", "one1one");

        re = re.replace("two", "two2two");

        re = re.replace("three", "three3three");

        re = re.replace("four", "four4four");

        re = re.replace("five", "five5five");

        re = re.replace("six", "six6six");

        re = re.replace("seven", "seven7seven");

        re = re.replace("eight", "eight8eight");

        re = re.replace("nine", "nine9nine");

        print!("Line result: {}", re);

        re.retain(|x| x.is_numeric());

        print!("There are the numbers: {}\n", re.to_string().as_str());
        let list: Vec<char> = re.chars().collect();
        let length = list.len();
        if length > 1 {
            let one: u32 = list[0].to_digit(10).unwrap();
            let two: u32 = list[length - 1].to_digit(10).unwrap();

            let mut sums: Vec<u32> = Vec::new();
            sums.push(one);
            sums.push(two);
            let s = concatinate_numbers(&sums);
            answer += s;

            print!("sum = {}, first number = {},number two = {}\n", s, one, two);
        } else {
            let mut sum: Vec<u32> = Vec::new();
            let one = list[0].to_digit(10).unwrap();
            sum.push(one);
            sum.push(one);

            let s = concatinate_numbers(&sum);

            print!("Sum of 1 digit: {}, Single Digit: {} \n", s + one, one);
            answer += s;
        }
    }
    print!("Answer = {}", answer);
}

fn concatinate_numbers(vec: &[u32]) -> u32 {
    vec.iter().fold(0, |acc, elem| acc * 10 + elem)
}
