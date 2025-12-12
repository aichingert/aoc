// TODO: just implement Io already
#include <stdio.h>

#define DAY "09"

struct Coord {
    s64 x;
    s64 y;
};

u64 part_one(Arena *allocator, String input) { 
    Coord *coords = NULL;
    s64 ans = 0;
    u32 pos = 0;

    while (pos < input.len) {
        s64 x = 0;
        s64 y = 0;

        while (pos < input.len && input.val[pos] != ',') {
            x = x * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && input.val[pos] != '\n') {
            y = y * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        Coord next = { .x = x, .y = y };
        array_push(allocator, coords, next);
    }

    for (u32 i = 0; i < array_len(coords); i++) {
        for (u32 j = i + 1; j < array_len(coords); j++) {
            ans = MAX(ans, ((1 + ABS(coords[i].x - coords[j].x)) * (1 + ABS(coords[i].y - coords[j].y))));
        }
    }

    return ans;
}

u64 part_two(Arena *allocator, String input) {
    Coord *coords = NULL;
    s64 ans = 0;
    u32 pos = 0;

    while (pos < input.len) {
        s64 x = 0;
        s64 y = 0;

        while (pos < input.len && input.val[pos] != ',') {
            x = x * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && input.val[pos] != '\n') {
            y = y * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        Coord next = { .x = x, .y = y };
        array_push(allocator, coords, next);
    }

    for (u32 i = 0; i < array_len(coords); i++) {
        Coord curr = coords[i];
        s64 top = -1;
        s64 bot = -1;
        s64 lhs = -1;
        s64 rhs = -1;

        printf("X: %lld     Y: %lld\n", curr.x, curr.y);

        for (u32 off = 1; off <= array_len(coords); off++) {
            Coord next = coords[(i + off) % array_len(coords)];

            if (curr.x == next.x) {
                // up ? down
                if (lhs == -1) {
                    lhs = curr.x;
                }
                if (rhs == -1) {
                    rhs = curr.x;
                }

                if (curr.y > next.y) {
                    // SET/TOP
                    //
                    //
                } else {

                }
            } else {
                // left ? right
                if (top == -1) {
                    top = curr.y;
                }
                if (bot == -1) {
                    bot = curr.y;
                }
            }

            printf("X: %lld     Y: %lld\n", next.x, next.y);
        }

        printf("TOP_Y: %lld     BOT_Y: %lld\n", top, bot);
        printf("LHS_X: %lld     RHS_X: %lld\n", lhs, rhs);
        printf("==================\n");
    }

    return ans;
}

// TODO: new version 
// provides global context
char **ENV;

u32 main(u32 argc, char **argv) {
    if (argc < 2) {
        printf("ERROR: specify which part to run\n");
        return 1;
    }
 
    String part = from_c_string(argv[1]);
    assert(
            part.len == 1 && (part.val[0] == '1' || part.val[0] == '2'), 
            S("ERROR: please provide a valid part e.g '1' or '2'"));

    Arena allocator = {0};
    arena_init(&allocator, 2 << 15);

    String input = file_read_as_string_alloc(&allocator, S("../input/2025/" DAY));
    u64 ans = 0;

    if (part.val[0] == '1') {
        ans = part_one(&allocator, input);
    } else {
        ans = part_two(&allocator, input);
    }

    printf("Answer [2025-" DAY "-%c]: %lu\n", part.val[0], ans);
    return 0;
}
