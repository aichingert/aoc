// Advent of Code 2018, day 1
// (c) aichingert

#include <iostream>
#include <vector>
#include <numeric>
#include <functional>
#include <set>
#include <fstream>
#include <string>

int part1(const std::vector<int>& a) {
  return std::accumulate(a.begin(), a.end(), 0);
}

int part2(const std::vector<int>& a) {
  int cur = 0;
  int i = 0;
  std::set<int> b {};

  while (b.insert(cur).second) {
    cur += a[i];
    i = (i + 1) % a.size();
  }

  return cur;
}

int main(void) {
  std::ifstream ifs("../input/01");
  std::vector<int> a {};
  std::string line {};

  while (std::getline(ifs, line)) {
    int idk = std::stoi(line);
    a.push_back(idk);
  }

  std::cout << "Part 1: "<< part1(a) << "\n";
  std::cout << "Part 2: "<< part2(a) << "\n";
}
