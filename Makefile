CC=cargo
CC_FLAG=--release
BIN=bzr
COMPRESS=upx
COMPRESS_FLAGS=--best --lzma
BUILD_LOCALE=./target/release

make:
	rm -rf ./target/
	${CC} build ${CC_FLAG}
	strip ${BUILD_LOCALE}/${BIN}

	