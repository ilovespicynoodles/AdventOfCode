use atoi::atoi;
use core::num;
use regex::Regex;
use std::fs;

const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let contents = fs::read_to_string("./inputs.txt");
    let mut games: Vec<String>;
    let mut amount = 0;
    match contents {
        Ok(con) => {
            games = con
                .lines()
                .map(|x| format!("{}\n", x))
                .collect::<Vec<String>>();
        }
        Err(_) => todo!(),
    }

    let true_games: Vec<u32> = Vec::new();
    for entry in games.iter_mut() {
        let grabs = entry.split(";");
        let mut current_game: u32 = 0;
        let mut red_grabs: u32 = 0;

        let mut green_grabs: u32 = 0;
        let mut blue_grabs: u32 = 0;
        let mut grab_is_under_max = true;
        grabs.for_each(|f| {
            // print!("{}\n", f);
            let colors = f.split(",");
            let word_g = "Game";
            colors.for_each(|c| {
                if c.contains("Game") {
                    let parsed_color = c.split(":");
                    parsed_color.for_each(|c_p| {
                        let without_whitespace = c_p.trim();
                        let entries: Vec<String> = without_whitespace
                            .split(" ")
                            .map(|s| s.to_string())
                            .collect();
                        if entries[0].contains("Game") {
                            let number = entries[1].parse().expect("Not a number.");
                            print!("Game Number: {}\n", number);
                            current_game = number;
                        }
                        if entries[1].contains("red") {
                            let number: u32 = entries[0].parse().expect("Not a number.");
                            print!("Red cubes: {}\n", number);
                            if number > MAX_RED {
                                grab_is_under_max = false
                            }
                            red_grabs = number;
                        }
                        if entries[1].contains("green") {
                            let number: u32 = entries[0].parse().expect("Not a number.");
                            print!("Green cubes: {}\n", number);
                            if number > MAX_GREEN {
                                grab_is_under_max = false
                            }
                            green_grabs = number;
                        }
                        if entries[1].contains("blue") {
                            let number: u32 = entries[0].parse().expect("Not a number.");
                            print!("blue cubes: {}\n", number);
                            if number > MAX_BLUE {
                                grab_is_under_max = false
                            }
                            blue_grabs = number;
                        }
                    })
                } else {
                    let without_whitespace = c.trim();
                    let entries: Vec<String> = without_whitespace
                        .split(" ")
                        .map(|s| s.to_string())
                        .collect();
                    if entries[1].contains("red") {
                        let number: u32 = entries[0].parse().expect("Not a number.");
                        print!("Red cubes: {}\n", number);
                        if number > MAX_RED {
                            grab_is_under_max = false
                        }
                        red_grabs = number;
                    }
                    if entries[1].contains("green") {
                        let number: u32 = entries[0].parse().expect("Not a number.");
                        print!("Green cubes: {}\n", number);
                        if number > MAX_GREEN {
                            grab_is_under_max = false
                        }
                        green_grabs = number;
                    }
                    if entries[1].contains("blue") {
                        let number: u32 = entries[0].parse().expect("Not a number.");
                        print!("blue cubes: {}\n", number);
                        if number > MAX_BLUE {
                            grab_is_under_max = false
                        }
                        blue_grabs = number;
                    }
                }

                // print!("{}\n", c);
                // let n = c.find(char::is_numeric).expect("no numbers found");
                // let game = Regex::new(r"/(Game \d\d\d)/gs")
                //     .expect("No game found")
                //     .find(c)
                //     .expect("Non found");
                // print!("Game thingy: {}", game.as_str());
                // // n.for_each(|numb| {
                // print!("Numbers :D = {}\n", n);
                // });
                // if c.contains("Game") {
                //     // let
                //     // c.find("Game");
                //     // let mut res = c.replace("Game ", "");
                //     let index = c.find("Game ").expect("Error");
                //     let number = c.get(index..).expect("No number");
                //     // res = res.replace(": ", "");
                //     // let number = c.trim().parse::<u32>().expect("No number found");
                //     print!("bnumber: {}\n", number);
                // }

                // if c.contains("red") {
                //     let mut res = c.replace("red ", "");
                //     res = res.replace(",", "");
                //     let number = res.trim().parse::<u32>().expect("No number found");
                //     print!("{}\n", number);
                // }
                // if c.contains("green") {
                //     let mut res = c.replace("green ", "");
                //     res = res.replace(",", "");
                //     let number = res.trim().parse::<u32>().expect("No number found");
                //     print!("{}\n", number);
                // }
                // if c.contains("blue") {
                //     let mut res = c.replace("blue ", "");
                //     // res = res.replace(",", "");
                //     print!("Blue issue: {}", res);
                //     let number = res.trim().parse::<u32>().expect("No number found");
                //     print!("{}\n", number);
                // }
            });
            print!(
                "Game: {} contains {} red, {} green and {} blue cubes.\n",
                current_game, red_grabs, green_grabs, blue_grabs
            );
            print!(".......\n");

            // if grab_is_under_max = false {}
        });
        if grab_is_under_max {
            amount += current_game;
        }
    }
    print!("Amount: {}", amount);
}
