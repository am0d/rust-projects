RUST_FLAGS = -L . -O

LIBS := 

ALL_SOURCES := $(wildcard *.rs)
BINARIES := binary-tree

ALL_BINARIES := $(BINARIES) $(ALL_SOURCES:%.rs=lib%)

BINARIES := $(filter-out $(LIBS), $(BINARIES))


all: $(BINARIES)

binary-tree: $(ALL_SOURCES)
	rustc $(RUST_FLAGS) binary-tree.rs

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..." 
	@rm -f *.so $(ALL_BINARIES)
