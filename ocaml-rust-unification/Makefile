t:
	cargo build

meta:
	cargo clean
	#	cargo rustc --verbose -- --emit=asm,llvm-bc,llvm-ir   --verbose
	RUSTFLAGS="--emit=llvm-ir --verbose" cargo build --verbose
