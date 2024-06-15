CC = gcc
CFLAGS = -Wall -Wextra -O2
TARGET = meowculator

all: $(TARGET)

$(TARGET): meowculator.c
	$(CC) $(CFLAGS) -o $@ $<

clean:
	rm -f $(TARGET)
