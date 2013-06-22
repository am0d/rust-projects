## Benchmarking rust applications.

#### Benchmarks implemented
The following sorting benchmarks have been implemented so far (I'm starting with the least efficient, and moving towards the most efficient):
* Swap sort
* Bubble sort
* Insertion sort
* Selection sort
* Merge sort
* Parallel merge sort

#### Command line flags supported
Each of the benchmark applications supports the following flags:
* `q` - Quiet operation (less information printed about each test)
* `qq` - No information printed while running the tests
* `trialsize` - The number of elements to be sorted in the benchmark
* `numtrials` - The number of trials to run

#### To compile:

    make

#### To run:

    $ ./swap-sort -qq
    Average time: 10,923 ns
    $ ./bubble-sort --numtrials 5 -q --trialsize 10000
    Trial 0
    Trial 1
    Trial 2
    Trial 3
    Trial 4
    Average time: 7.43483 sec

#### Libraries
These applications only rely on the standard library functionality.
However, I have created 2 libraries of code which are shared between the programs.

The first is `libtimer`, which is a small wrapper around the built in nano-second timer.
It allows me to start and stop a timer, calculate the elapsed time, and pretty print that elapsed time.

The second is `libbenchmark`, which handles things such as parsing command line arguments, creating the random arrays, etc.
