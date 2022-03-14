built_compiler: jilox scanner errors
	cc -g -O0 -Wall -ledit target/jilox.o target/scanner.o target/errors.o -o target/jilox.out

jilox:
	cc -g -O0 -Wall -c jilox.c
	mv jilox.o target/

scanner:
	cc -g -O0 -Wall -c scanner.c
	mv scanner.o target/

errors:
	cc -g -O0 -Wall -c errors.c
	mv  errors.o target/




