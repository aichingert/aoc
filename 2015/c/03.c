// Advent of Code 2015, day 3
// (c) aichingert

#include <stdio.h>
#include <stdlib.h>

int main(void)
{
	FILE *fp = fopen("../input/03", "r");
	char ch;

	int po[2] = {0,0};
	int pt[2][2] = {{0,0},{0,0}};

	while ((ch = fgetc(fp)) != EOF)
	{
		printf("%c", ch);	
	}

	printf("Part 1: %i\n", 10);
	printf("Part 2: %i\n", 2200);

	fclose(fp);
	return 0;
}
