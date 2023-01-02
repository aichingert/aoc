// Utility - stack 
// (c) aichingert

typedef struct {
	int data;
	struct node *next;
} node;

typedef struct {
	node *top;
	size_t malloc_size;
} stack;

void push(stack *stack, void *data);
