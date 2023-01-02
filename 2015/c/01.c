// Advent of Code 2015, day 1
// (c) aichingert

#include <stdio.h>
#include <stdlib.h>

int main(void) 
{
	FILE *fp = fopen("../input/01", "r");
	char ch;
	int loc = 0;
	int part = 1;
	int iter = 0;

	while ((ch = fgetc(fp)) != EOF)
	{
		if (part) 
		{
			iter += 1;
		}

		if (ch == '(') 
		{
			loc += 1;
		} else if (ch == ')') 
		{
			loc -= 1;
		}

		if (part && loc < 0) 
		{
			part = 0;
		}
	}

	printf("Part 1: %i\n", loc);
	printf("Part 2: %i\n", iter);

	fclose(fp);
	return 0;
}
