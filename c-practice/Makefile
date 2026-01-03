CC      := gcc
CFLAGS := -g -Og -Wall -Wextra -Wpedantic -std=c11 \
          -Wformat=2 -Wshadow -Wconversion -Wcast-align \
          -Wnull-dereference -Wdouble-promotion

BUILD   := build

.PHONY: run clean

run:
	@if [ -z "$(p)" ]; then \
		echo "Usage: make run p=<c-practice-0n>"; \
		exit 1; \
	fi
	@mkdir -p $(BUILD)
	$(CC) $(CFLAGS) $(wildcard $(p)/src/*.c) -o $(BUILD)/a.out
	./$(BUILD)/a.out

clean:
	rm -f $(BUILD)/a.out
