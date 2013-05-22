all: bubble-sort

bubble-sort: libbenchmark bubble-sort.rs
	rustc bubble-sort.rs -L .

libbenchmark: benchmark.rs libtimer
	rustc --lib benchmark.rs -L .

libtimer: timer.rs
	rustc --lib timer.rs

clean:
	rm libbenchmark* bubble-sort
