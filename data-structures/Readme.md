Benchmarking rust applications.

Benchmarks implemented:
    * Swap sort
    * Bubble sort

Command line flags supported:
    * q - Quiet operation (less information printed about each test)
    * qq - Extra quite operation
    * qqq - No information printed while running the tests
    * trialsize - The number of elements to be sorted in the benchmark
    * numtrials - The number of trials to run

To compile:

    make

To run:

    $ ./swap-sort -qqq
    Average time: 10,923 ns
    $ ./bubble-sort --numtrials 5 -q --trialsize 10000
    Trial 0
    Trial 1
    Trial 2
    Trial 3
    Trial 4
    Average time: 7.43483 sec
