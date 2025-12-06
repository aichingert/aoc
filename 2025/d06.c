// TODO: just implement Io already
#include <stdio.h>

#define DAY "06"

u64 part_one(Arena *allocator, String input) { 
    u32 *nums = NULL;
    u8 *ops = NULL;
    u32 pos = 0;
    u64 ans = 0;

    while (pos < input.len && (input.val[pos] == ' ' || input.val[pos] == '\n')) {
        pos += 1;
    }

    while (pos < input.len && input.val[pos] != '+' && input.val[pos] != '*') {
        u32 num = 0;

        while (pos < input.len && input.val[pos] >= '0' && input.val[pos] <= '9') {
            num = num * 10 + input.val[pos] - '0';
            pos += 1;
        }

        array_push(allocator, nums, num);

        while (pos < input.len && (input.val[pos] == ' ' || input.val[pos] == '\n')) {
            pos += 1;
        }
    }

    while (pos < input.len) {
        if (input.val[pos] == '+' || input.val[pos] == '*') {
            array_push(allocator, ops, input.val[pos]);
        }

        pos += 1;
    }

    for (u32 i = 0; i < array_len(ops); i++) {
        u64 acc = nums[i];
        for (u32 j = i + array_len(ops); j < array_len(nums); j += array_len(ops)) {
            if (ops[i] == '+') {
                acc = acc + nums[j];
            } else {
                acc = acc * nums[j];
            }
        }

        ans += acc;
    }

    return ans;
}

u64 part_two(Arena *allocator, String input) {
    u32 *nums = NULL;
    u32 width = 0;
    u32 times = 0;
    u32 pos = 0;
    u64 ans = 0;

    while (width < input.len && input.val[width] != '\n') {
        width += 1;
    }
    width += 1;
    assert(width < input.len, S("invalid input"));

    while (pos < input.len && input.val[pos] != '*' && input.val[pos] != '+') {
        pos += width;
    }
    assert(pos < input.len && (input.val[pos] == '+' || input.val[pos] == '*'), S("invalid input expected op"));
    times = pos / width;

    pos = 0;

    for (s32 i = width - 1; i >= 0; i--) {
        u32 num = 0;
        bool has_digit = false;

        for (u32 j = 0; j < times; j++) {
            if (input.val[i + j * width] >= '0' && input.val[i + j * width] <= '9') {
                has_digit = true;
                num = num * 10 + input.val[i + j * width] - '0';
            }
        }

        if (has_digit) {
            array_push(allocator, nums, num);
            u8 op = input.val[width * times + i];
            u64 acc = nums[0];

            if  (op == '+' || op == '*') {
                for (u32 i = 1; i < array_len(nums); i++) {
                    if (op == '+') {
                        acc += nums[i];
                    } else {
                        acc *= nums[i];
                    }
                }
                ans += acc;
                array_clear(nums);
            }
        }
    }

    return ans;
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
