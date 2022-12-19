#include <bits/stdc++.h>

using namespace std;

int main(int argc, char *argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    string line;
    while (getline(f, line)) { cout << line << endl; }
}
