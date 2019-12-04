use std::fs;

fn main() {
    day1();
    day2();
}

fn day1(){
    let directions = fs::read_to_string("input.txt").unwrap();

    let wires = directions.split("\n").collect::<Vec<&str>>();

    let wire1 = wires[0].trim().to_string();
    let wire2 = wires[1].trim().to_string();

    let wire1_pos = calculate_path(wire1);
    let wire2_pos = calculate_path(wire2);

    let mut shortest = 999999999;

    for w1 in wire1_pos{
        for w2 in &wire2_pos{
            if w1 == *w2{
                let distance = calculate_manhattan_distance(0, 0, w2.0, w2.1);

                if distance < shortest{
                    shortest = distance;
                    println!("shortest: {}", shortest);
                }
            }
        }
    }

    println!("final: {}", shortest);
}

fn day2(){
    let directions = fs::read_to_string("input.txt").unwrap();

    let wires = directions.split("\n").collect::<Vec<&str>>();

    let wire1 = wires[0].trim().to_string();
    let wire2 = wires[1].trim().to_string();

    let wire1_pos = calculate_path(wire1);
    let wire2_pos = calculate_path(wire2);

    let mut shortest = 999999999;

    let mut wire1_steps = 0;
    for w1 in wire1_pos{
        wire1_steps += 1;
        let mut wire2_steps = 0;
        for w2 in &wire2_pos{
            wire2_steps += 1;
            if w1 == *w2{
                let distance = wire1_steps + wire2_steps;

                if distance < shortest{
                    shortest = distance;
                    println!("shortest: {}", shortest);
                }
            }
        }
    }

    println!("final: {}", shortest);
}

fn calculate_path(wire: String) -> Vec<(i32,i32)>{
    let directions = wire.split(",").collect::<Vec<&str>>();
    let mut path_pos: Vec<(i32, i32)> = Vec::new();

    let mut x = 0;
    let mut y = 0;

    for direction in directions{
        let dir = direction.chars().nth(0).unwrap();
        let steps_string: String = direction.chars().skip(1).take(direction.len()).collect();
        let steps: i32 = steps_string.parse().unwrap();

        for _ in 0..steps{
            if dir == 'U' {
                y += 1;
            }
            if dir == 'R' {
                x += 1;
            }
            if dir == 'D' {
                y -= 1;
            }
            if dir == 'L' {
                x -= 1;
            }

            path_pos.push((x, y));
        }
    }

    return path_pos;
}

fn calculate_manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32{
    return (x1 - x2).abs() + (y1 - y2).abs();
}