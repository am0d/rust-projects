use std::os;

fn fact(n: int) -> int {
    let mut result = 1;
    let mut i = 1;
    while i <= n {
        result *= i;
        i += 1;
    }

    result
}

fn main() {
    let args = os::args();
    if args.len() >= 2 {
        let val = match from_str::<int>(args[1]) {
            Some(n) => { n}
            _ => {
                println("n must be an integer");
                0
            }
        };
            
        let val= fact(val);
        println!("{}", val);
    }
    else {
        println!("Usage: {} n", args[0]);
    }
}
