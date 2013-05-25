RUST_FLAGS = -L . -O

all: swap-sort bubble-sort selection-sort

swap-sort: libbenchmark swap-sort.rs
	rustc $(RUST_FLAGS) swap-sort.rs

bubble-sort: libbenchmark bubble-sort.rs
	rustc $(RUST_FLAGS) bubble-sort.rs

selection-sort: libbenchmark selection-sort.rs
	rustc $(RUST_FLAGS) selection-sort.rs

libbenchmark: benchmark.rs libtimer
	rustc $(RUST_FLAGS) benchmark.rs

libtimer: timer.rs
	rustc $(RUST_FLAGS) timer.rs

clean:
	rm *.so *-sort
