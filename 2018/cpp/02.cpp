#include <iostream>
#include <vector>
#include <numeric>
#include <functional>
#include <map>
#include <fstream>
#include <string>

int part1(const std::vector<std::string>& a) {
  int tw = 0, th = 0;

  for (int i = 0; i < a.size(); i++) {
    std::map<char, int> counter = {};

    for (int j = 0; j < a[i].length(); j++) {
      if (!counter.contains(a[i][j])) {
        counter[a[i][j]] = 0;
      }      

      counter[a[i][j]]++;
    }

    bool w = false;
    bool h = false;

    for (auto& [_, value] : counter) {
      if (value == 2) { w = true; }
      if (value == 3) { h = true; }
    }

    if (w) { tw++; }
    if (h) { th++; }
  }
   
  return tw * th;
}

int part2(const std::vector<std::string>& a) {
  
  return 0;
}

int main(void) {
  std::ifstream ifs("../input/02");
  std::vector<std::string> a {};
  std::string line {};

  while (std::getline(ifs, line)) {
    a.push_back(line);
  }

  std::cout << "Part 1: "<< part1(a) << "\n";
  std::cout << "Part 2: "<< part2(a) << "\n";
}