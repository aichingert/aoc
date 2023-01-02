// Advent of Code 2015, day 2
// (c) aichingert

#include <stdio.h>
#include <stdlib.h>

#define MAX_LINE_LENGTH 15
#define SIZE 3

int cmp(const void *a, const void *b)
{
	return ( *(int*)a - *(int*)b );
}

void get_numbers(char line[MAX_LINE_LENGTH], int nums[SIZE])
{
	int idx = 0;
	int n = 0;

	for(int i = 0; i < MAX_LINE_LENGTH; ++i)
	{
		if (line[i] == '\n') 
			break;

		if (line[i] != 'x')
		{
			n *= 10;
			n += line[i] - '0';
		}
		else
		{
			nums[idx++] = n;
			n = 0;
		}
	}
	nums[idx] = n;
	qsort(nums, SIZE, sizeof(int), cmp);
}

int main(void)
{
	FILE *fp = fopen("../input/02", "r");
	char line[MAX_LINE_LENGTH];
	int paper = 0;
	int ribbon = 0;

	while (fgets(line, MAX_LINE_LENGTH, fp))
	{
		int nums[SIZE];
		get_numbers(line,nums);

		paper += 2*nums[0]*nums[1] + 2*nums[1]*nums[2] + 2*nums[0]*nums[2] + nums[0]*nums[1];
		ribbon += 2*nums[0] + 2*nums[1] + nums[0]*nums[1]*nums[2];
	}

	printf("Part 1: %i\n", paper);
	printf("Part 2: %i\n", ribbon);

	fclose(fp);
	return 0;
}
