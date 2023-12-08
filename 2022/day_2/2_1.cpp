#include <bits/stdc++.h>

#include <unordered_map>

using namespace std;

int result(char a, char b) {
    if ((a == 'A' && b == 'X') ||
        (a == 'B' && b == 'Y') ||
        (a == 'C' && b == 'Z')) {
        return 3;
    }

    if ((a == 'A' && b == 'Y') ||
        (a == 'B' && b == 'Z') ||
        (a == 'C' && b == 'X')) {
        return 6;
    }

    return 0;
}

int main() {
    ifstream f;
    f.open("./input.txt");

    string line;
    unordered_map<char, int> score;

    score['A'] = 1;
    score['B'] = 2;
    score['C'] = 3;

    score['X'] = 1;
    score['Y'] = 2;
    score['Z'] = 3;

    int ss = 0;
    while (getline(f, line)) {
        // cout << line << endl;

        char o = line[0];
        char i = line[2];

        int curr = score[i] + result(o, i);

        // cout << score[i] << " " << result(o, i) << " "
        //      << curr << endl;
        ss += curr;
    }

    cout << ss;
}
