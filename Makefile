ASM=assembly.rs

all: about_*.rs
	cp header.file $(ASM)
	ls -1 about* | sed -e 's/^ab/mod ab/' | sed -e 's/\.rs/;/g' >>$(ASM)
	rustc $(ASM) --test -o koans
	rm $(ASM)

run: all
	./koans
