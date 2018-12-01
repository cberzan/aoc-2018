use std::io;

fn main() {
    let mut result = 0;
    let mut line = String::new();
    loop {
        line.clear();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        if line.is_empty() {
            break;
        }
        let line_int: i32 = line.trim().parse()
            .expect("Could not convert to i32");
        result += line_int;
    }
    println!("{}", result);
}
