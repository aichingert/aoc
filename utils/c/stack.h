// Utils - Stack
// (c) aichingert

#include <stdlib.h>
#include <string.h>

struct node;
struct stack;

struct stack *new_stack(size_t malloc_size);
void s_push(struct stack *stack, void *data);
int s_pop(struct stack *stack, void *data);
int s_get_at(struct stack *stack, int index, void *data);
int s_get_len(struct stack *stack);
