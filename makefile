syanpse: synapse.o Scanner.o
	g++ -o synapse synapse.o Scanner.o

synapse.o: synapse.cpp Scanner.h
	g++ -c synapse.cpp

Scanner.o: Scanner.cpp Scanner.h Token.cpp Token.h
	g++ -c Scanner.cpp

Token.o: Token.cpp Token.h
	g++ -c Token.cpp

clean:
	rm -f *.o synapse
