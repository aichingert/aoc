// Advent of Code 2015, day 5
// (c) aichingert

#include <stdio.h>

#define MAX_LINE_LENGTH 18

int is_vowel(char ch)
{
	if (ch == 'a' || ch == 'e' || ch == 'i' || ch == 'o' || ch == 'u') return 1;
	return 0;
}

int part1(char *line)
{
	int vwls = 0;
	if (is_vowel(line[MAX_LINE_LENGTH-2])) vwls++;
	int dd = 0;
	int inv = 1;

	for (int i = 0; i < MAX_LINE_LENGTH-2; ++i) 
	{
		if (is_vowel(line[i])) vwls++;

		if (line[i] == 'a' && line[i+1] == 'b' 
		|| line[i] == 'c' && line[i+1] == 'd' 
		|| line[i] == 'p' && line[i+1] == 'q'
		|| line[i] == 'x' && line[i+1] == 'y')
		{
			inv = 0;
			break;
		}

		if (line[i] == line[i+1]) { dd++; }
	}

	if (vwls > 2 && dd && inv) return 1;

	return 0;
}

int part2(char *line)
{
	int i = -1;
	int dd = 0;
	int zdd = 0;

	while (i++ < MAX_LINE_LENGTH-3)
	{
		for (int j = i+2; j < MAX_LINE_LENGTH-2; ++j)
		{
			if (line[i] == line[j] && line[i+1] == line[j+1])
			{
				dd = 1;
				break;
			}
		}

		if (line[i] == line[i+2]) zdd++;

		if (zdd && dd) break;
	}

	if (zdd && dd) return 1;
	return 0;
}

int main(void)
{
	FILE *fp = fopen("../input/05", "r");
	char line[MAX_LINE_LENGTH];
	int p1 = 0;
	int p2 = 0;

	while (fgets(line, MAX_LINE_LENGTH, fp))
	{
		p1 += part1(line);
		p2 += part2(line);
	}

	printf("Part 1: %d\n", p1);
	printf("Part 2: %d\n", p2);

	return 0;
}
	
