
all: libhello.a

hello.o: hello.c
	gcc -c hello.c -o hello.o

libhello.a: hello.o
	ar rcs libhello.a hello.o

clean:
	rm libhello.a hello.o

