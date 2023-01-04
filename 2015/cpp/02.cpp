// Advent of Code 2015, day 2
// (c) aichingert

#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <bits/stdc++.h>

std::vector<int> get(std::string line)
{
	std::vector<int> n;
	std::string cur;

	for (int i = 0; i < line.length(); ++i)
	{
		if (line[i] == '\n') break;
		if (line[i] == 'x') 
		{
			n.push_back(std::stoi(cur));
			cur = "";
		}
		else 
		{
			cur += line[i];
		}

	}
	
	if (line != "") n.push_back(std::stoi(cur));

	std::sort(n.begin(), n.end());
	return n;
}

int main(void)
{
	std::ifstream fp ("../input/02");
	std::string inp;
	int paper = 0;
	int ribbon = 0;

	if (fp.is_open())
	{
		while (fp)
		{
			std::getline(fp,inp);
			std::vector n = get(inp);

			if (n.size() > 0)
			{
				paper += 2*n[0]*n[1] + 2*n[1]*n[2] + 2*n[0]*n[2] + n[0]*n[1];
				ribbon += 2*n[0] + 2*n[1] + n[0]*n[1]*n[2];
			}
		}
	}

	std::cout << "Part 1: " << paper << "\n";
	std::cout << "Part 2: " << ribbon << "\n";

	return 0;
}
