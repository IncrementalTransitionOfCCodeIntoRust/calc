TARGET := calc
C_SOURCES := $(shell ls *.c)
CC := gcc
C_FLAGS := -lpthread -Wl,--no-as-needed -ldl -lm
RUST_LIB = rust/target/debug/librustcalc.a

all:
	cd rust && cargo test && cargo build --verbose
	$(CC) $(C_FLAGS) $(C_SOURCES) $(RUST_LIB) -o $(TARGET)

clean:
	rm -f $(TARGET) && cd rust && cargo clean
