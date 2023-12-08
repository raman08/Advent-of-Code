#include <bits/stdc++.h>

using namespace std;

int main(int argc, char *argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;
    while (getline(file, line)) { cout << line << endl; }
	cout << endl;
}
