fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::with_capacity(num_rows as usize);
    for i in 0..num_rows {
        let count = i + 1;
        let temp = vec![1; count as usize];
        result.push(temp);
        if i != 0 && i != 1 {
            for j in 1..i {
                result[i as usize][j as usize] =
                    result[i as usize - 1][j as usize - 1] + result[i as usize - 1][j as usize]
            }
        }
    }
    result
}

fn main() {
    let result = generate(10);
    for i in result.iter() {
        for j in (*i).iter() {
            print!("{}", j);
        }
        println!();
    }
}
