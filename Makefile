TARGET = helloworld.bin

AS = nasm

LDFLAGS += --entry=func --oformat=binary

.PHONY: all
all :
	make $(TARGET)


%.bin : %.asm Makefile
	$(AS) -f bin -o $@ $<
