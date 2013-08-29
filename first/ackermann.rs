use std::io;
use std::os;
use std::int;

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
        io::println(fmt!("usage: %s m n", args[0]));
        return;
    };

    let m = match int::from_str(args[1]) {
        Some(x) => {x}
        _ => {io::println(fmt!("m must be an integer"));
              return;
             }
    };

    let n = match int::from_str(args[2]) {
        Some(x) => {x}
        _ => {io::println(fmt!("n must be an integer"));
              return;
             }
    };

    let result = ackermann(m, n);

    io::println(fmt!("ackermann(%d, %d)", m, n));
    io::println(fmt!("Result: %d", result));
}
