// Util - stack
// (c) aichingert

#include <stdio.h>

typedef struct {
	void *data;
	struct node *next;
} node;

typedef struct {
	node *top;
	size_t malloc_size;
} stack;

void push(stack *stack, void *data)
{
	if (stack == NULL)
	{

	}
}
