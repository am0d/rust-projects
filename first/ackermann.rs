use std::os;

fn ackermann(m: int, n: int) -> int {
    if m == 0 {
        n + 1
    }
    else {
        if n == 0 {
            ackermann(m - 1, 1)
        }
        else {
            ackermann(m - 1, ackermann(m, n-1))
        }
    }
}

fn main() {
    let args = os::args();
    if args.len() < 3 {
        println!("usage: {} m n", args.get(0));
        return;
    };

    let m = match from_str::<int>(args.get(1).as_slice()) {
        Some(x) => {x}
        _ => {
            println!("m must be an integer");
            return;
        }
    };

    let n = match from_str::<int>(args.get(2).as_slice()) {
        Some(x) => {x}
        _ => {
            println!("n must be an integer");
            return;
        }
    };

    let result = ackermann(m, n);

    println!("ackermann({}, {})", m, n);
    println!("Result: {}", result);
}
