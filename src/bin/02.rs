advent_of_code::solution!(2);

pub fn checkstr(numbers: Vec<&str>) -> bool{
    let mut is_ordered = true;
    let sign;
    let mut correct_steps = true;
    if numbers.len() <= 1{
        sign = true;
    }else{
        if numbers[0].parse::<i32>().unwrap() < numbers[1].parse().unwrap(){
            sign = true;
        }else{
            sign = false;
        }
    }
    for i in 0..(numbers.len()-1) {
        if sign{
            if numbers[i].parse::<i32>().unwrap() > numbers[i+1].parse().unwrap(){
                is_ordered = false;
            }
        }else{
            if numbers[i].parse::<i32>().unwrap() < numbers[i+1].parse().unwrap() {is_ordered = false}
        }
        if (numbers[i].parse::<i32>().unwrap() - numbers[i+1].parse::<i32>().unwrap()).abs() > 3 || numbers[i].parse::<i32>().unwrap() == numbers[i+1].parse().unwrap(){
            correct_steps = false;
        }
    }
    if is_ordered && correct_steps{
        true
    }else{
        false
    }

}
pub fn part_one(input: &str) -> Option<u32> {
    let rows: Vec<&str> = input.split("\n").collect();
    let mut out: u32 = 0;
    println!("{}",rows.len());
    for row in rows{
        let numbers: Vec<&str> = row.split(" ").collect();
        if checkstr(numbers.clone()){
            out += 1;
        }else{
            for i in 0..numbers.len(){
                let mut numberclone = numbers.clone();
                numberclone.remove(i);
                if checkstr(numberclone){
                    out+= 1;
                    break;
                }
            }
        }
    }
    Some(out)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
