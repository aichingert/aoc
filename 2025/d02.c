// NOTE: not sure if I like this implementation yet...
// currently I do not really

// TODO: just implement Io already
#include <stdio.h>

// TODO: new version 
// provides global context
char **ENV;

u64 part_one(Arena *allocator, String input) {
    u64 ans = 0;
    u32 pos = 0;

    while (pos < input.len) {
        u64 beg = 0;
        u64 end = 0;

        while (pos < input.len && input.val[pos] != '-') {
            beg = beg * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && (input.val[pos] >= '0' && input.val[pos] <= '9')) {
            end = end * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        for (; beg <= end; beg++) {
            String s = to_string_u64(allocator, beg);

            u64 f_half = 0;
            u64 s_half = 0;

            for (u32 i = 0; i < s.len / 2; i++) {
                f_half = f_half * 10 + s.val[i] - '0';
            }
            for (u32 i = s.len / 2; i < s.len; i++) {
                s_half = s_half * 10 + s.val[i] - '0';
            }

            if (f_half == s_half && s.len % 2 == 0) {
                ans += beg;
            }
        }
    }

    return ans;
}

u64 part_two(Arena *allocator, String input) {
    u64 ans = 0;
    u32 pos = 0;

    while (pos < input.len) {
        u64 beg = 0;
        u64 end = 0;

        while (pos < input.len && input.val[pos] != '-') {
            beg = beg * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && (input.val[pos] >= '0' && input.val[pos] <= '9')) {
            end = end * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        for (; beg <= end; beg++) {
            String s = to_string_u64(allocator, beg);
            bool done = false;

            for (u32 i = 1; i <= s.len / 2 && !done; i ++) {
                if (s.len % i != 0) {
                    continue;
                }

                u32 size = s.len / i;
                bool is_valid = true;

                for (u32 j = 0; j < size && is_valid; j++) {
                    for (u32 k = 0; k < i; k++) {
                        if (s.val[k] != s.val[k + i * j]) {
                            is_valid = false;
                            break;
                        }
                    }
                }


                if (is_valid) {
                    ans += beg;
                    done = true;
                    break;
                }
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

    String input = file_read_as_string_alloc(&allocator, S("../input/2025/02"));
    u64 ans = 0;

    if (part.val[0] == '1') {
        ans = part_one(&allocator, input);
    } else {
        ans = part_two(&allocator, input);
    }

    printf("Answer [2025-02-%c]: %lu\n", part.val[0], ans);
    return 0;
}
