// TODO: just implement Io already
#include <stdio.h>

#define DAY "08"

struct Coord {
    s64 x;
    s64 y;
    s64 z;
};

struct Pair {
    u32 i;
    u32 j;
    f64 dist;
};

struct NumSet {
    u32 key;
};

struct Graph {
    u32 *nodes;
};

void merge(Arena *allocator, Pair *pairs, u32 l, u32 m, u32 r) {
    u32 i, j, k;
    u32 n1 = m - l + 1;
    u32 n2 = r - m;

    Pair *L = alloc(allocator, Pair, n1);
    Pair *R = alloc(allocator, Pair, n2);

    for (i = 0; i < n1; i++) {
        L[i] = pairs[l + i];
    }
    for (j = 0; j < n2; j++) {
        R[j] = pairs[m + 1 + j];
    }

    i = 0;
    j = 0;
    k = l;
    while (i < n1 && j < n2) {
        if (L[i].dist <= R[j].dist) {
            pairs[k] = L[i];
            i += 1;
        } else {
            pairs[k] = R[j];
            j += 1;
        }
        
        k += 1;
    }
    while (i < n1) {
        pairs[k] = L[i];
        k += 1;
        i += 1;
    }
    while (j < n2) {
        pairs[k] = R[j];
        k += 1;
        j += 1;
    }
}

void merge_sort(Arena *allocator, Pair *pairs, u32 l, u32 r) {
    if (l < r) {
        u32 m = l + (r - l) / 2;

        merge_sort(allocator, pairs, l, m);
        merge_sort(allocator, pairs, m + 1, r);
        merge(allocator, pairs, l, m, r);
    }
}

f64 distance(Coord a, Coord b) {
    return hsqrt((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y) + (a.z - b.z) * (a.z - b.z));
}

u64 find(Arena *allocator, NumSet **seen, Graph *graph, u32 node) {
    if (hm_get(*seen, node) != NULL) {
        return 0;
    }
    NumSet empty = {0};
    hm_put(allocator, *seen, node, empty);

    u64 ans = 1;
    for (u32 i = 0; i < array_len(graph[node].nodes); i++) {
        ans += find(allocator, seen, graph, graph[node].nodes[i]);
    }

    return ans;
}

u64 part_one(Arena *allocator, String input) {
    Pair *pairs = NULL;
    Coord *coords = NULL;
    u32 pos = 0;

    while (pos < input.len) {
        s64 x = 0;
        s64 y = 0;
        s64 z = 0;

        while (pos < input.len && input.val[pos] != ',') {
            x = x * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && input.val[pos] != ',') {
            y = y * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && input.val[pos] != '\n') {
            z = z * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        Coord next = { .x = x, .y = y, .z = z };
        array_push(allocator, coords, next);
    }

    for (u32 i = 0; i < array_len(coords); i++) {
        for (u32 j = i + 1; j < array_len(coords); j++) {
            Pair p = { .i = i, .j = j, .dist = distance(coords[i], coords[j]) };
            array_push(allocator, pairs, p);
        }
    }

    
    merge_sort(allocator, pairs, 0, array_len(pairs) - 1);

    Graph *graph = alloc(allocator, Graph, array_len(pairs));
    NumSet *seen = NULL;
    NumSet empty = {0};
    u32 key = array_len(pairs);
    hm_put(allocator, seen, key, empty);

    for (u32 i = 0; i < MIN(1000, array_len(pairs)); i++) {
        array_push(allocator, graph[pairs[i].i].nodes, pairs[i].j);
        array_push(allocator, graph[pairs[i].j].nodes, pairs[i].i);
    }

    u64 circuits[3] = {0, 0, 0};

    for (u32 i = 0; i < array_len(pairs); i++) {
        if (array_len(graph[i].nodes) == 0) {
            continue;
        }
        u64 circuit = find(allocator, &seen, graph, i);

        if          (circuits[0] < circuit) {
            circuits[2] = circuits[1];
            circuits[1] = circuits[0];
            circuits[0] = circuit;
        } else if   (circuits[1] < circuit) {
            circuits[2] = circuits[1];
            circuits[1] = circuit;
        } else if   (circuits[2] < circuit) {
            circuits[2] = circuit;
        }
    }

    return circuits[0] * circuits[1] * circuits[2];
}

u64 part_two(Arena *allocator, String input) {
    Pair *pairs = NULL;
    Coord *coords = NULL;
    u32 pos = 0;

    while (pos < input.len) {
        s64 x = 0;
        s64 y = 0;
        s64 z = 0;

        while (pos < input.len && input.val[pos] != ',') {
            x = x * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && input.val[pos] != ',') {
            y = y * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;
        while (pos < input.len && input.val[pos] != '\n') {
            z = z * 10 + input.val[pos] - '0';
            pos += 1;
        }
        pos += 1;

        Coord next = { .x = x, .y = y, .z = z };
        array_push(allocator, coords, next);
    }

    for (u32 i = 0; i < array_len(coords); i++) {
        for (u32 j = i + 1; j < array_len(coords); j++) {
            Pair p = { .i = i, .j = j, .dist = distance(coords[i], coords[j]) };
            array_push(allocator, pairs, p);
        }
    }
    
    merge_sort(allocator, pairs, 0, array_len(pairs) - 1);

    Graph *graph = alloc(allocator, Graph, array_len(pairs));
    
    for (u32 i = 0; i < array_len(pairs); i++) {
        array_push(allocator, graph[pairs[i].i].nodes, pairs[i].j);
        array_push(allocator, graph[pairs[i].j].nodes, pairs[i].i);

        NumSet *seen = NULL;
        NumSet empty = {0};
        u32 key = array_len(pairs);
        hm_put(allocator, seen, key, empty);
        if (find(allocator, &seen, graph, 0) == array_len(coords)) {
            return coords[pairs[i].i].x * coords[pairs[i].j].x;
        }
    }

    return 0;
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
