RUST_FLAGS = -L . -O
ALL_SOURCES := $(wildcard *.rs)
BINARIES := $(ALL_SOURCES:.rs=)
BINARIES += $(ALL_SOURCES:%.rs=lib%)

all: swap-sort bubble-sort selection-sort merge-sort merge-sort-par

swap-sort: libbenchmark swap-sort.rs
	rustc $(RUST_FLAGS) swap-sort.rs

bubble-sort: libbenchmark bubble-sort.rs
	rustc $(RUST_FLAGS) bubble-sort.rs

selection-sort: libbenchmark selection-sort.rs
	rustc $(RUST_FLAGS) selection-sort.rs

merge-sort: libbenchmark merge-sort.rs
	rustc $(RUST_FLAGS) merge-sort.rs

merge-sort-par: libbenchmark merge-sort-par.rs
	rustc $(RUST_FLAGS) merge-sort-par.rs

libbenchmark: benchmark.rs libtimer
	rustc $(RUST_FLAGS) benchmark.rs
	@#rustc $(RUST_FLAGS) --test -o $@ $<
	@touch $@

libtimer: timer.rs
	rustc $(RUST_FLAGS) timer.rs
	@#rustc $(RUST_FLAGS) --test -o $@ $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f *.so $(BINARIES)
