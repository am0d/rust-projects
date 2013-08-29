RUST_FLAGS = -L . -O

LIBS := 

ALL_SOURCES := $(wildcard *.rs)
BINARIES := $(ALL_SOURCES:.rs=)

ALL_BINARIES := $(BINARIES) $(ALL_SOURCES:%.rs=lib%)

BINARIES := $(filter-out $(LIBS), $(BINARIES))


all: $(BINARIES)

%: %.rs 
	rustc $(RUST_FLAGS) $<

lib%: %.rs
	rustc $(RUST_FLAGS) $<
	@touch $@

clean:
	@echo "Cleaning ..."
	@rm -f *.so $(ALL_BINARIES)
