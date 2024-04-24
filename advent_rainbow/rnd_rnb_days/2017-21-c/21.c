#include <stdio.h>
#include <stdlib.h>

#define LINES 130
#define LINE_SIZE 50

struct {
    int dimensions;
    char **map;
} typedef t_square;

t_square
create_square(int dimensions) {
    char **map;
    map = malloc(dimensions * sizeof *map);

    for (int i = 0; i < dimensions; ++i) {
        map[i] = malloc(dimensions * sizeof *map[i]);
    }

    return (t_square) {
        .dimensions = dimensions,
        .map = map,
    };
}

t_square*
split_square(t_square square) {
    // TODO: implement split logic
    return NULL;
}

t_square
merge_square(t_square* squares, int size) {
    // TODO: implement merge logic
    int dimensions = size * squares[0].dimensions;

    return (t_square) { .dimensions = dimensions, .map = NULL };
}

void
part_one(t_square start, char** rules) {
    // TODO: implement part one

    for (int i = 0; i < LINES; ++i) {
        if (rules[i] == NULL) break;
        printf("%s", rules[i]);
    }
}

int 
main(void) {
    FILE *f = fopen("../../input/2017/21", "r");

    if (f == NULL) {
        printf("ERROR: unable to open file\n");
        return 1;
    }

    int i = 0;
    char **rules;
    char line[LINE_SIZE];

    rules = malloc(LINES * sizeof *rules);
    rules[i++] = malloc(LINE_SIZE * sizeof *rules[i - 1]);

    while (fgets(rules[i - 1], LINE_SIZE, f))
        rules[i++] = malloc(LINE_SIZE * sizeof *rules[i - 1]);

    t_square start = create_square(3);
    start.map[0][0] = '.';
    start.map[0][1] = '#';
    start.map[0][2] = '.';
    start.map[1][0] = '.';
    start.map[1][1] = '.';
    start.map[1][2] = '#';
    start.map[2][0] = '#';
    start.map[2][1] = '#';
    start.map[2][1] = '#';

    part_one(start, rules);

    for (int i = 0; i < LINES; ++i) free(rules[i]);
    free(rules);

    return 0;
}
