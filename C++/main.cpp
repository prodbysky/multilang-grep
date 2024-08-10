#include <fstream>
#include <iostream>
#include <string>

int main(int argc, char *argv[]) {
  if (argc < 3) {
    std::cerr << "Not enough arguments provided!\n";
    return 1;
  }

  const char *file = argv[1];
  const char *needle = argv[2];

  std::ifstream f(file);

  std::string line;

  while (std::getline(f, line)) {
    if (line.find(needle) != std::string::npos) {
      std::cout << line << '\n';
    }
  }

  return 0;
}
