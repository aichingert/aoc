// Advent of Code 2015, day 4
// (c) aichingert

#include <stdio.h>
#include <stdlib.h>
#include "../../utils/c/md5.h"

#define MAX_LINE_LENGTH 15

int solve(char* key, int zeros)
{
	int n = 0;
	int d = 1;

	while (1)
	{
		n += 1;
		char buf[20];
		char vls[10];

		// Getting hash
		snprintf(buf, 20, "%s%d", key, n);
		uint8_t *hash = md5String(buf);
		d = 1;

		// storing first X characters
		snprintf(vls, 10, "%02X%02X%02X", hash[0], hash[1],hash[2]);

		// checking if one is not zero
		for (int i = 0; i < zeros; ++i)
		{
			if (vls[i] != '0')
			{
				d = 0;
				break;
			}
		}

		if (d)
		{
			break;
		}
	}

	return n;
}

int main(void) {
	FILE *fp = fopen("../input/04", "r");
	char key[MAX_LINE_LENGTH];
	fgets(key, MAX_LINE_LENGTH, fp);

	printf("Part 1: %d\n", solve(key, 5));
	printf("Part 2: %d\n", solve(key, 6));

	fclose(fp);
}
