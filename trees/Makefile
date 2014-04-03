RUST_FLAGS = -L . -O

LIBS := 

ALL_SOURCES := $(wildcard *.rs)
BINARIES := binarytree

all: $(BINARIES)

run: $(BINARIES)
	./binarytree

binarytree: $(ALL_SOURCES)
	rustc $(RUST_FLAGS) binarytree.rs

clean:
	@echo "Cleaning ..." 
	@rm -f $(BINARIES)
