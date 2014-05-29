ASM=assembly.rs

all: about_*.rs lib
	cp header.file $(ASM)
	ls -1 about* | sed -e 's/^ab/mod ab/' | sed -e 's/\.rs/;/g' >>$(ASM)
	rustc $(ASM) --test -o koans -L./resources
	rm $(ASM)

lib:
	cd resources; make; cd ..
	mv resources/libid.so .

run: all
	./koans | python script/pretty-print.py
