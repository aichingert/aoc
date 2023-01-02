// Advent of Code 2015, day 3
// (c) aichingert

#include <stdio.h>
#include <stdlib.h>
#include "../../utils/c/stack.h"

void update(int p[2], char cur)
{
	switch (cur)
	{
		case '>': p[0] += 1; break;
		case '<': p[0] -= 1; break;
		case '^': p[1] += 1; break;
		case 'v': p[1] -= 1; break;
	}
}

int not_contained(struct stack *stack, int data[2])
{
	int contained = 1;
	int len = s_get_len(stack);
	int check[2] = {0,0};

	for (int i = 0; i < len; ++i)
	{
		s_get_at(stack, i, &check);
		if (check[0] == data[0] && check[1] == data[1])
		{
			contained = 0;
			break;
		}
	}

	return contained;
}

int main(void)
{
	FILE *fp = fopen("../input/03", "r");
	char ch;

	int po[2] = {0,0};
	int pt[2][2] = {{0,0},{0,0}};
	int loc = 0;
	struct stack *part_one = new_stack(sizeof(po));
	struct stack *part_two = new_stack(sizeof(po));
	int data[2];
	s_push(part_one, &po);
	s_push(part_two, &po);

	while ((ch = fgetc(fp)) != EOF)
	{
		update(po, ch);
		update(pt[loc], ch);
		if (not_contained(part_one, po)) s_push(part_one, &po);
		if (not_contained(part_two, pt[loc])) s_push(part_two, &pt[loc]);
		loc = 1 - loc;
	}

	printf("Part 1: %d\n", s_get_len(part_one));
	printf("Part 2: %i\n", s_get_len(part_two));

	fclose(fp);
	return 0;
}
