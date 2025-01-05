CC = gcc
CFLAGS = -Wall -Wextra -std=c11
TARGET = micromake
SRCS = Micromake.c

all: $(TARGET)

$(TARGET): $(SRCS)
	$(CC) $(CFLAGS) -o $(TARGET) $(SRCS)

clean:
	rm -f $(TARGET)