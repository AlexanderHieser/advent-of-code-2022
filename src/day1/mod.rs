use std::fs;
pub fn run(path: &str) {
    println!("Hello Day1 Args {}", path);
    let lines = fs::read_to_string(path).expect("Could not read txt file");
    let mut lines:Vec<_> = lines.split("\n\n").map(|line:&str| -> usize {
        println!("{}",line);
        let sum:Vec<usize> = line.split("\n").map(|line:&str| -> usize {
            let u:usize = line.parse().unwrap();
            u
        }).collect();
        let sum = sum.iter().sum();
        println!("Sum {:?}", sum);
        sum
    }).collect();

    // Little bit ugly but works
    lines.sort();
    lines.reverse();
    println!("Max: {}", lines.get(0).unwrap());
    println!("Max 3: {}", get_top_3(&lines));
}

fn get_top_3(sums: &Vec<usize>) -> usize{
    let mut count:usize = 0;
    for n in 0..3  {
        println!("{}", sums[n]);
        count = count + sums[n];
    }
    count
}