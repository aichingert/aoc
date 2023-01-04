// Advent of Code 2015, day 3
// (c) aichingert

#include <iostream>
#include <fstream>
#include <string>
#include <unordered_set>

// This was 30 minutes googling
namespace std {
template <> struct hash<std::pair<int, int>> {
	inline size_t operator()(const std::pair<int, int> &v) const {
		std::hash<int> int_hasher;
		return int_hasher(v.first) ^ int_hasher(v.second);
	}
};
}

void update(int (&loc)[2], char ch)
{
	switch (ch)
	{
		case '<':
			loc[0] -= 1;
			break;
		case '>':
			loc[0] += 1;
			break;
		case '^':
			loc[1] += 1;
			break;
		case 'v':
			loc[1] -= 1;
			break;
	}
}

int solve(std::string inp, int part)
{
	
	std::unordered_set<std::pair<int,int>> set;
	int loc[part+1][2];
	int cur = 0;

	for (int i = 0; i < part+1; i++)
		for (int j = 0; j < 2; j++)
			loc[i][j] = 0;
	set.insert(std::make_pair(loc[0][0],loc[0][1]));

	for (int i = 0; i < inp.length(); i++)
	{
		update(loc[cur],inp[i]);
		set.insert(std::make_pair(loc[cur][0],loc[cur][1]));

		if (part) cur = 1 - cur;
	}
		

	return set.size();
}

int main(void) 
{
	std::ifstream fp ("../input/03");
	std::string inp;

	if (fp.is_open())
		std::getline(fp,inp);

	std::cout << "Part 1: " << solve(inp, 0) << "\n";
	std::cout << "Part 2: " << solve(inp, 1) << "\n";
	return 0;
}
