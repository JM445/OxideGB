##
# Oxide GB Test ROM
#
# @file
# @version 0.1

SRCS=test.asm
OBJS=$(SRCS:.asm=.o)

OUT=test.gb

all: $(OUT)

$(OUT): $(OBJS)
	rgblink -d -o $(OUT) $(OBJS)

%.o: %.asm
	rgbasm $^ -o $@
# end
