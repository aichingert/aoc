// TODO: just implement Io already
#include <stdio.h>

#define DAY "10"

struct NumArr {
    u64 *nums;
};

struct LineOne {
    u64 pattern;
    NumArr *buttons;
};

struct Bfs {
    u64 state;
    u64 depth;
};

struct NumSet {
    u64 key;
};

void skip_whitespace(u32 *pos, String source) {
    while (*pos < source.len && source.val[*pos] == ' ') {
        *pos += 1;
    }
}

u64 part_one(Arena *allocator, String input) {
    u32 pos = 0;
    u64 ans = 0;

    while (pos < input.len) {
        assert(input.val[pos] == '[', S("invalid input"));
        pos += 1;
        u32 beg = pos;

        LineOne line = {0};

        while (pos < input.len && input.val[pos] != ']') {
            line.pattern |= (input.val[pos] == '#') << (pos - beg);
            pos += 1;
        }
        assert(pos < input.len && input.val[pos] == ']', S("invalid input"));
        pos += 1;

        skip_whitespace(&pos, input);

        while (pos < input.len && input.val[pos] != '{') {
            pos += 1;

            NumArr arr = {0};
            while (pos < input.len && input.val[pos] != ')') {
                u64 num = 0;

                while (pos < input.len && input.val[pos] >= '0' && input.val[pos] <= '9') {
                    num = num * 10 + input.val[pos] - '0';
                    pos += 1;
                }

                array_push(allocator, arr.nums, num);
                if (pos < input.len && input.val[pos] == ')') {
                    pos += 1;
                    break;
                }
                pos += 1;
            }

            array_push(allocator, line.buttons, arr);
            skip_whitespace(&pos, input);
        }

        while (pos < input.len && input.val[pos] != '\n') {
            pos += 1;
        }
        pos += 1;

        Bfs *bfs = NULL;
        NumSet *seen = NULL;
        NumSet none = {0};
        u32 i = 0;
        array_push(allocator, bfs, ((Bfs){ .state = 0, .depth = 0 }));

        while (i < array_len(bfs)) {
            if (hm_get(seen, bfs[i].state) != NULL) {
                i += 1;
                continue;
            }
            if (bfs[i].state == line.pattern) {
                ans += bfs[i].depth;
                break;
            }
            hm_put(allocator, seen, bfs[i].state, none);

            for (u32 j = 0; j < array_len(line.buttons); j++) {
                u64 curr = bfs[i].state;

                for (u32 k = 0; k < array_len(line.buttons[j].nums); k++) {
                    curr ^= (1 << line.buttons[j].nums[k]);
                }

                array_push(allocator, bfs, ((Bfs){ .state = curr, .depth = bfs[i].depth + 1}));
            }

            i += 1;
        }
    }

    return ans;
}

u64 part_two(Arena *allocator, String input) {
    u32 pos = 0;
    u64 ans = 0;

    while (pos < input.len) {
        assert(input.val[pos] == '[', S("invalid input"));
        pos += 1;

        while (pos < input.len && input.val[pos] != ']') {
            pos += 1;
        }
        assert(pos < input.len && input.val[pos] == ']', S("invalid input"));
        pos += 1;

        LineOne line = {0};
        skip_whitespace(&pos, input);

        while (pos < input.len && input.val[pos] != '{') {
            pos += 1;

            NumArr arr = {0};
            while (pos < input.len && input.val[pos] != ')') {
                u64 num = 0;

                while (pos < input.len && input.val[pos] >= '0' && input.val[pos] <= '9') {
                    num = num * 10 + input.val[pos] - '0';
                    pos += 1;
                }

                array_push(allocator, arr.nums, num);
                if (pos < input.len && input.val[pos] == ')') {
                    pos += 1;
                    break;
                }
                pos += 1;
            }

            array_push(allocator, line.buttons, arr);
            skip_whitespace(&pos, input);
        }

        while (pos < input.len && input.val[pos] != '\n') {
            pos += 1;
        }
        pos += 1;
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
