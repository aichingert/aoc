// Utils - Stack
// (c) aichingert

#include <stdlib.h>
#include <string.h>

struct node 
{
	void *data;
	struct node *next;
};

struct stack 
{
	struct node *top;
	size_t malloc_size;
	int len;
};

struct stack *new_stack(size_t malloc_size)
{
	struct stack *stack = malloc(sizeof(struct stack));
	stack->top = NULL;
	stack->malloc_size = malloc_size;
	stack->len = 0;
	
	return stack;
}

void s_push(struct stack *stack, void *data)
{
	struct node *n = malloc(sizeof(struct node));
	n->data = malloc(stack->malloc_size);
	memcpy(n->data, data, stack->malloc_size);

	n->next = stack->top;
	stack->top = n;
	stack->len += 1;
}

int s_pop(struct stack *stack, void *data)
{
	if (stack == NULL || stack->top == NULL)
	{
		return -1;
	}

	struct node *n = stack->top;
	stack->top = n->next;
	stack->len -= 1;

	memcpy(data, n->data, stack->malloc_size);
	free(n->data);
	free(n);

	return 0;
}

int s_get_at(struct stack *stack, int index, void *data) 
{
	if (stack == NULL || stack->top == NULL)
	{
		return -1;
	}

	struct node *n = stack->top;

	for (int i = 0; i < index; ++i)
	{
		n = n->next;

		if (n == NULL)
		{
			return -1;
		}
	}

	memcpy(data,n->data,stack->malloc_size);
	return 0;
}

int s_get_len(struct stack *stack)
{
	return stack->len;
}
