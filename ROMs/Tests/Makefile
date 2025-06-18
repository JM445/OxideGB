##
# Oxide GB Test ROM
#
# @file
# @version 0.1

# SRCS=test.asm
# OBJS=$(SRCS:.asm=.o)

# OUT=test.gb

OUT_TEST = test.gb
OUT_CPU_TESTS = cpu_tests.gb

all: $(OUT_TEST) $(OUT_CPU_TESTS)

$(OUT_TEST): test.o
	rgblink -d -o $(OUT_TEST) test.o

$(OUT_CPU_TESTS): cpu_tests.o
	rgblink -d -o $(OUT_CPU_TESTS) cpu_tests.o

%.o: %.asm
	rgbasm -o $@ $^
# end
