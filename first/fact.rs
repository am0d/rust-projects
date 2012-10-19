extern mod std;

fn fact(n: int) -> int {
    let mut result = 1, i = 1;
    while i <= n {
        result *= i;
        i += 1;
    }

    result
}

fn main() {
    let args = os::args();
    if args.len() >= 2 {
        let val = match int::from_str(args[1]) {
            Some(n) => { n}
            _ => {
                io::println("n must be an integer");
                0
            }
        };
            
        let val= fact(val);
        io::println(#fmt("%d", val));
    }
}
