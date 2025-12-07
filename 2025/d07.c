// TODO: just implement Io already
#include <stdio.h>

#define DAY "07"

u64 part_one(Arena *allocator, String input) {
    u32 width = 0;
    u64 ans = 0;
    u32 pos = 0;

    String str = str_copy(allocator, input, 0, input.len);

    while (width < str.len && str.val[width] != '\n') {
        width += 1;
    }
    width += 1;

    while (pos < str.len) {
        for (u32 i = 0; i < width; i++) {
            if          (str.val[pos + i] == 'S') {
                str.val[pos + i + width] = '|';
            } else if   (str.val[pos + i] == '|') {
                if          (pos + i + width < str.len && str.val[pos + i + width] == '^') {
                    ans += 1;

                    if (i > 0 && str.val[pos + i + width - 1] == '.') {
                        str.val[pos + i + width - 1] = '|';
                    } 
                    if (i + 1 < width && str.val[pos + i + width + 1] == '.') {
                        str.val[pos + i + width + 1] = '|';
                    }
                } else if   (pos + i + width < str.len && str.val[pos + i + width] == '.') {
                    str.val[pos + i + width] = '|';
                }
            }
        }

        pos += width;
    }

    return ans;
}

u64 find(String input, u32 width, u64 *mem, u32 beam) {
    if (beam > input.len) {
        return 1;
    }

    u64 ans = 0;
    u32 below = beam + width;

    if (below < input.len && input.val[below] == '^') {
        if (mem[below] > 0) {
            return mem[below];
        }

        if (input.val[below - 1] != '\n') {
            ans += find(input, width, mem, below - 1);
        } 
        if (below + 1 < input.len && input.val[below + 1] != '\n') {
            ans += find(input, width, mem, below + 1);
        }

        mem[below] = ans;
        return ans;
    }

    return find(input, width, mem, below);
}

u64 part_two(Arena *allocator, String input) {
    u32 beam = 0;
    u32 width = 0;

    while (width < input.len && input.val[width] != '\n') {
        if (input.val[width] == 'S') {
            beam = width;
        }
        width += 1;
    }
    width += 1;
    assert(beam > 0, S("expecting the starting position to be in the top row"));

    u64 *mem = alloc(allocator, u64, input.len);
    return find(input, width, mem, beam);
}

// TODO: new version 
// provides global context
char **ENV;

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
