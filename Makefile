all:
	cargo build --release

clean:
	cargo clean

fclean: clean

re: fclean all

.PHONY: all clean fclean re
