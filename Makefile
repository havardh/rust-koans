all: about_*.rs
	cp header.file koans.rs
	ls -1 about* | sed -e 's/^ab/mod ab/' | sed -e 's/\.rs/;/g' >>koans.rs
	rustc koans.rs --test -o koans
	rm koans.rs

run: all
	./koans
