// TODO: just implement Io already
#include <stdio.h>

// TODO: new version 
// provides global context
char **ENV;

u64 part_one(String input) {
    u64 ans = 0;
    u32 pos = 0;

    while (pos < input.len) {
        u8 jolt[2] = {0, 0};

        while (pos + 1 < input.len && input.val[pos + 1] >= '0' && input.val[pos + 1] <= '9') {
            if          (jolt[0] < input.val[pos] - '0') {
                jolt[0] = input.val[pos] - '0';
                jolt[1] = 0;
            } else if   (jolt[1] < input.val[pos] - '0') {
                jolt[1] = input.val[pos] - '0';
            }

            pos += 1;
        }

        jolt[1] = MAX(jolt[1], input.val[pos] - '0');
        pos += 2;
        ans += jolt[0] * 10 + jolt[1];
    }

    return ans;
}

u64 part_two(String input) {
    // get biggest num in bank_len - amount
    u64 ans = 0;
    u32 pos = 0;

    const u32 batteries = 12;

    while (pos < input.len) {
        u32 size = pos;
        u32 left = 0;
        u64 bank = 0;

        while (pos < input.len && input.val[pos] >= '0' && input.val[pos] <= '9') {
            pos += 1;
        }
        size = pos - size;

        if (size == 0) {
            pos += 1;
            continue;
        }

        pos -= size;

        for (u32 i = 0; i < batteries; i++) {
            u8 jolts = '0';
            u32 high = left;

            while (left <= size - batteries + i) {
                if (jolts < input.val[pos + left]) {
                    jolts = input.val[pos + left];
                    high = left;
                }
                left += 1;
            }

            left = high + 1;
            bank = bank + (jolts - '0') * pow(10, batteries - i - 1);
        }

        pos += size + 1;
        ans += bank;
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

    String input = file_read_as_string_alloc(&allocator, S("../input/2025/03"));
    u64 ans = 0;

    if (part.val[0] == '1') {
        ans = part_one(input);
    } else {
        ans = part_two(input);
    }

    printf("Answer [2025-03-%c]: %lu\n", part.val[0], ans);
    return 0;
}
