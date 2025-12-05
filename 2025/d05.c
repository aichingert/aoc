// TODO: just implement Io already
#include <stdio.h>

struct Range {
    u64 beg;
    u64 end;
};

void sort_ranges(Range *ranges) {
    for (u32 i = 0; i < array_len(ranges) - 1; i++) {
        for (u32 j = i + 1; j < array_len(ranges); j++) {
            if (ranges[i].beg > ranges[j].beg || (ranges[i].beg == ranges[j].beg && ranges[i].end > ranges[j].end)) {
                Range tmp = ranges[i];
                ranges[i] = ranges[j];
                ranges[j] = tmp;
            }
        }
    }
}

u64 part_one(Arena *allocator, String input) {
    Range *ranges = NULL;
    u32 pos = 0;
    u64 ans = 0;

    while (pos < input.len && input.val[pos] != '\n') {
        Range rng = {
            .beg = 0,
            .end = 0,
        };

        while (pos < input.len && input.val[pos] != '-') {
            rng.beg = rng.beg * 10 + input.val[pos] - '0';
            pos += 1;
        }

        assert(pos < input.len && input.val[pos] == '-', S("expected '-' to seperate range"));
        pos += 1;

        while (pos < input.len && input.val[pos] != '\n') {
            rng.end = rng.end * 10 + input.val[pos] - '0';
            pos += 1;
        }

        assert(pos < input.len && input.val[pos] == '\n', S("expected '\n' to end range"));
        pos += 1;

        array_push(allocator, ranges, rng);
    }
    pos += 1;

    sort_ranges(ranges);
    Range *merged = NULL;
    array_push(allocator, merged, ranges[0]);
    u32 merged_pos = 0;

    for (u32 i = 1; i < array_len(ranges); i++) {
        if (merged[merged_pos].end >= ranges[i].beg) {
            merged[merged_pos].end = MAX(merged[merged_pos].end, ranges[i].end);
        } else {
            array_push(allocator, merged, ranges[i]);
            merged_pos += 1;
        }
    }

    while (pos < input.len && input.val[pos] != '\n') {
        u64 id = 0;

        while (pos < input.len && input.val[pos] != '\n') {
            id = id * 10 + input.val[pos] - '0';
            pos += 1;
        }
        assert(pos < input.len && input.val[pos] == '\n', S("expected '\n' to end id"));
        pos += 1;

        for (u32 i = 0; i < array_len(merged); i++) {
            if (id >= merged[i].beg && id <= merged[i].end) {
                ans += 1;
                break;
            }
        }
    }

    return ans;
}

u64 part_two(Arena *allocator, String input) {
    Range *ranges = NULL;
    u32 pos = 0;
    u64 ans = 0;

    while (pos < input.len && input.val[pos] != '\n') {
        Range rng = {
            .beg = 0,
            .end = 0,
        };

        while (pos < input.len && input.val[pos] != '-') {
            rng.beg = rng.beg * 10 + input.val[pos] - '0';
            pos += 1;
        }

        assert(pos < input.len && input.val[pos] == '-', S("expected '-' to seperate range"));
        pos += 1;

        while (pos < input.len && input.val[pos] != '\n') {
            rng.end = rng.end * 10 + input.val[pos] - '0';
            pos += 1;
        }

        assert(pos < input.len && input.val[pos] == '\n', S("expected '\n' to end range"));
        pos += 1;

        array_push(allocator, ranges, rng);
    }
    pos += 1;

    sort_ranges(ranges);
    Range *merged = NULL;
    array_push(allocator, merged, ranges[0]);
    u32 merged_pos = 0;

    for (u32 i = 1; i < array_len(ranges); i++) {
        if (merged[merged_pos].end >= ranges[i].beg) {
            merged[merged_pos].end = MAX(merged[merged_pos].end, ranges[i].end);
        } else {
            ans += merged[merged_pos].end - merged[merged_pos].beg;
            array_push(allocator, merged, ranges[i]);
            merged_pos += 1;
        }
    }

    return ans + merged[merged_pos].end - merged[merged_pos].beg + array_len(merged);
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

    String input = file_read_as_string_alloc(&allocator, S("../input/2025/05"));
    u64 ans = 0;

    if (part.val[0] == '1') {
        ans = part_one(&allocator, input);
    } else {
        ans = part_two(&allocator, input);
    }

    printf("Answer [2025-05-%c]: %lu\n", part.val[0], ans);
    return 0;
}
