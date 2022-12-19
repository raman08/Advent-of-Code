#include <bits/stdc++.h>

#include <unordered_map>

using namespace std;

int result(char a, char b) {
    if (a == 'A') {
        if (b == 'X') {
            return 3;
        } else if (b == 'Y') {
            return 1;
        } else {
            return 2;
        }
    }

    if (a == 'B') {
        if (b == 'X') {
            return 1;
        } else if (b == 'Y') {
            return 2;
        } else {
            return 3;
        }
    }

    if (b == 'X') {
        return 2;
    } else if (b == 'Y') {
        return 3;
    } else {
        return 1;
    }
}

int main() {
    ifstream f;
    f.open("./input.txt");

    string line;
    unordered_map<char, int> score;

    score['A'] = 1;
    score['B'] = 2;
    score['C'] = 3;

    score['X'] = 0;
    score['Y'] = 3;
    score['Z'] = 6;

    int ss = 0;
    while (getline(f, line)) {
        // cout << line << endl;

        char o = line[0];
        char i = line[2];

        int curr = score[i] + result(o, i);

        // cout << score[i] << " " << result(o, i) <<
        // " "
        //      << curr << endl;
        ss += curr;
    }

    cout << ss;
}
