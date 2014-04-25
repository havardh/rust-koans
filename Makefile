all: about_*.rs
	cat about_*.rs >koans.rs 
	rustc koans.rs --test -o koans
	rm koans.rs

run: all
	./koans
