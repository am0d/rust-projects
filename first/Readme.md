### First projects


In this directory is a sample of small projects that I first wrote while learning rust.
I have added a few as time goes on, and have done / do my best to ensure that all the
programs compile with a fairly recent version of Rust.

To build all the programs in this directory, run:

    make clean
    make

The projects are as follows:

* `hello.rs` - A small Hello-World program which prints "Hello world from '<arg1>'", where arg1 is the first command line argument (the program name)
* `hello-x.rs` - A small Hello-world program which takes input from the user and echos that back to the screen.
* `hello-multithreaded.rs` - A small rust program to demonstrate multithreading.  20 child process are spawned, which all send a message back to the parent process.  The parent process echoes to the screen the child id from which it recieved a message.  Notice that the messages are not always received in order.
* `ackermann.rs` - A small program to calculate the Ackermann numbers.  Demonstrates recursion in rust.  Note, it is not recommended to run the program with `m > 3`.
* `fact.rs` - Calculates the factorial of a number provided on the command line.
* `prime-sieve.rs` - A concurrent prime sieve.  This is a port of the go project at http://play.golang.org/p/9U22NfrXeq, using Rust channels and threads instead of go-channels.
