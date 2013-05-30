RUST_FLAGS = -L . -O

LIBS := benchmark timer

ALL_SOURCES := $(wildcard *.rs)
BINARIES := $(ALL_SOURCES:.rs=)

ALL_BINARIES := $(BINARIES) $(ALL_SOURCES:%.rs=lib%)

BINARIES := $(filter-out $(LIBS), $(BINARIES))


all: $(BINARIES)

%: %.rs libbenchmark
	rustc $(RUST_FLAGS) $<

libbenchmark: libtimer

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f *.so $(ALL_BINARIES)
