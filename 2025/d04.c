// TODO: just implement Io already
#include <stdio.h>

// TODO: new version 
// provides global context
char **ENV;

u64 part_one(String input) {
    u64 ans = 0;
    u32 width = 0;
    u32 pos = 0;

    while (width < input.len && input.val[width] != '\n') {
        width += 1;
    }
    width += 1;

    while (pos < input.len) {
        for (u32 i = 0; i < width - 1; i++) {
            if (input.val[pos + i] == '@') {
                u8 cnt = 0;

                for (s8 j = -1; j < 2; j++) {
                    for (s8 k = -1; k < 2; k++) {
                        s32 idx = (pos + i) + width * j + k;

                        if (idx > -1 && (u32)idx < input.len && input.val[idx] == '@') {
                            cnt += 1;
                        }
                    }
                }

                if (cnt < 5) {
                    ans += 1;
                }
            }
        }

        pos += width;
    }

    return ans;
}

u64 part_two(String input) {
    u64 ans = 0;
    u32 width = 0;
    u32 pos = 0;
    bool has_removed = true;

    while (width < input.len && input.val[width] != '\n') {
        width += 1;
    }
    width += 1;

    while (has_removed) {
        pos = 0;
        has_removed = false;

        while (pos < input.len) {
            for (u32 i = 0; i < width - 1; i++) {
                if (input.val[pos + i] == '@') {
                    u8 cnt = 0;

                    for (s8 j = -1; j < 2; j++) {
                        for (s8 k = -1; k < 2; k++) {
                            s32 idx = (pos + i) + width * j + k;

                            if (idx > -1 
                            && (u32)idx < input.len 
                            && (input.val[idx] == '@' || input.val[idx] == 'X')) {
                                cnt += 1;
                            }
                        }
                    }

                    if (cnt < 5) {
                        ans += 1;
                        has_removed = true;
                        input.val[pos + i] = 'X';
                    }
                }
            }

            pos += width;
        }

        for (u32 i = 0; i < input.len; i++) {
            if (input.val[i] == 'X') {
                input.val[i] = 'x';
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

    String input = file_read_as_string_alloc(&allocator, S("../input/2025/04"));
    u64 ans = 0;

    if (part.val[0] == '1') {
        ans = part_one(input);
    } else {
        ans = part_two(input);
    }

    printf("Answer [2025-04-%c]: %lu\n", part.val[0], ans);
    return 0;
}
