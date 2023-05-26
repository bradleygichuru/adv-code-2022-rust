use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
fn main() {
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./adv-2022-day-1") {
        let mut current_elf_calorie_count = 0;
        let mut elf_calories: Vec<i32> = Vec::new();
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(calorie) = line {
                println!("{}", calorie);
                if calorie.is_empty() {
                    elf_calories.push(current_elf_calorie_count);
                    current_elf_calorie_count = 0;
                } else {
                    current_elf_calorie_count =
                        current_elf_calorie_count + calorie.parse::<i32>().unwrap();
                }
            }
        }
        let max_calories_collected =  elf_calories.iter().max().unwrap();
        let index_of_elf_with_max_calories = elf_calories.iter().position(|&x| x == max_calories_collected.clone()).unwrap();
        println!("Length of elf_calories vector: {}",elf_calories.len());
        println!("{}",elf_calories[index_of_elf_with_max_calories]);
        println!("The elf with the most calories is the {:?}th elf with {} calories",index_of_elf_with_max_calories+1,max_calories_collected);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
