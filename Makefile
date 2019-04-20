all:
	cargo build --release

run: 
	cargo run --release

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re