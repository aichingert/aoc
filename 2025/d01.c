// TODO: just implement Io already
#include <stdio.h>

// TODO: new version 
// provides global context
char **ENV;

#define DIAL_SIZE   100U

u32 mod(s32 val, u32 m) {
    s32 modulo = val % (s32)m;
    if (modulo < 0) {
        modulo += m;
    }

    return modulo;
}

u32 part_one(String input) { 
    u32 pos = 0;
    u32 ans = 0;
    s32 cur = 50;

    while (pos < input.len) {
        s32 sign = input.val[pos] == 'L' ? -1 : 1;
        s32 dist = 0;

        pos += 1;
        while (pos < input.len && input.val[pos] != '\n') {
            dist *= 10;
            dist += input.val[pos] - '0';
            pos += 1;
        }

        pos += 1;

        cur = mod(cur + (sign * dist), DIAL_SIZE);
        if (cur == 0) {
            ans += 1;
        }
    }

    return ans;
}

u32 part_two(String input) {
    u32 pos = 0;
    u32 ans = 0;
    s32 cur = 50;

    while (pos < input.len) {
        s32 sign = input.val[pos] == 'L' ? -1 : 1;
        s32 dist = 0;

        pos += 1;
        while (pos < input.len && input.val[pos] != '\n') {
            dist *= 10;
            dist += input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        ans += dist / DIAL_SIZE;
        dist %= DIAL_SIZE;

        // TODO: do some math brah
        if (sign == 1) {
            while (dist > 0) {
                if (cur == 99) {
                    cur = 0;
                    ans += 1;
                } else {
                    cur += 1;
                }

                dist -= 1;
            }
        } else {
            while (dist > 0) {
                if (cur == 1) {
                    ans += 1;
                }
                if (cur == 0) {
                    cur = 99;
                } else {
                    cur -= 1;
                }

                dist -= 1;
            }
        }
    }

    return ans;
}

s32 main(s32 argc, char **argv) {
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

    String input = file_read_as_string_alloc(&allocator, S("../input/2025/01"));
    u32 ans = 0;

    if (part.val[0] == '1') {
        ans = part_one(input);
    } else {
        ans = part_two(input);
    }

    printf("Answer [2025-01-%c]: %u\n", part.val[0], ans);
    return 0;
}
