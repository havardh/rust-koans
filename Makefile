all: about_*.rs
	ls -1 about* | sed -e 's/^/mod /' | sed -e 's/\.rs/;/g' >koans.rs 
	rustc koans.rs --test -o koans
	rm koans.rs

run: all
	./koans
