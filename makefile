OUT = bin\flux.exe
SRC = src/*.c src/*/*.c src/*/*/*.c src/*/*/*/*.c

LIBS = -lSDL2main -lSDL2 -lSDL2_ttf -lSDL2_mixer -lSDL2_image
FLAGS = -std=c11 -Llibs/lib -Ilibs/include -g

$(OUT): $(SRC)
	gcc -o $(OUT) $(SRC) $(LIBS) $(FLAGS)

.PHONY: run
run: $(OUT)
	./$(OUT)

.PHONY: clean
clean:
	del .\$(OUT)