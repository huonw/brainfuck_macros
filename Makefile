RUSTC ?= rustc
RUSTFLAGS ?= -O

OUT=build

DYLIB = $(shell $(RUSTC) --crate-file-name lib.rs)
YO_DAWG = $(shell $(RUSTC) --crate-file-name bf_bf_interpreter.rs)

.PHONY: all test yo_dawg lib

all: lib test

$(OUT):
	mkdir -p $(OUT)

lib:  $(OUT) $(OUT)/$(DYLIB)

yo_dawg: $(OUT)/$(YO_DAWG)

test: $(OUT)/test.stamp

$(OUT)/test.stamp: $(OUT)/test
	$(OUT)/test
	-touch $(OUT)/test.stamp


$(OUT)/$(DYLIB) : lib.rs
	$(RUSTC) $(RUSTFLAGS) --out-dir $(OUT) lib.rs

$(OUT)/$(YO_DAWG) : $(OUT)/$(DYLIB) bf_bf_interpreter.rs
	$(RUSTC) $(RUSTFLAGS) --out-dir $(OUT) bf_bf_interpreter.rs -L $(OUT)

$(OUT)/test : $(OUT)/$(DYLIB) $(OUT)/$(YO_DAWG) test.rs
	$(RUSTC) $(RUSTFLAGS) --test --out-dir $(OUT) test.rs --cfg bf_bf_interpreter -L $(OUT)
