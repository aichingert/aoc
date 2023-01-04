// Advent of Code 2015, day 1
// (c) aichingert

#include <iostream>
#include <fstream>
#include <string>

int solve(std::string inp, int part)
{
	int loc = 0;

	for (int i = 0; i < inp.length(); ++i)
	{
		if (inp[i] == '(') loc += 1;
		if (inp[i] == ')') loc -= 1;

		if (part && loc < 0) return i + 1;
	}

	return loc;
}

int main(void)
{
	std::ifstream fp ("../input/01");
	std::string inp;

	if (fp.is_open())
	{
		std::getline(fp,inp);
	}

	std::cout << "Part 1: " << solve(inp, 0) << "\n";
	std::cout << "Part 2: " << solve(inp, 1) << "\n";

	return 0;
}
