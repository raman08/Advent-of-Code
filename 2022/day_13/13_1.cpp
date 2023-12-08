#include <bits/stdc++.h>

#include <cstdio>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <vector>

using namespace std;

vector<string> tokenize(string s) {
    vector<string> tokens;
    int idx = 1;
    string token = "";
    while (idx < s.size()) {
        if (idx == s.size() - 1) {
            if (!token.empty()) {
                tokens.push_back(token);
                token = "";
            }
            break;
        }
        if (s[idx] == '[') {
            token += '[';

            int opening = 1;
            idx++;
            while (true) {
                switch (s[idx]) {
                    case '[': opening++; break;
                    case ']': opening--; break;
                }
                token += s[idx];
                if (opening == 0) break;
                idx++;
            }

            tokens.push_back(token);

            token = "";
            idx++;

            continue;
        }
        if (s[idx] == ',') {
            if (!token.empty()) {
                tokens.push_back(token);
                token = "";
            }
            idx++;
            continue;
        }

        token += s[idx];
        idx++;
    }

    return tokens;
}

int matchVec(vector<string> v1, vector<string> v2) {
    for (int i = 0; i < v1.size(); i++) {
        if (i >= v2.size()) return 1;

        string s1 = v1[i];
        string s2 = v2[i];

        stringstream ss1(s1);
        stringstream ss2(s2);

        int n1, n2;
        if (ss1 >> n1 && ss2 >> n2) {
            if (n1 < n2) { return -1; }
            if (n1 > n2) { return 1; }
        } else {
            if (s1[0] != '[') { s1 = '[' + s1 + ']'; }
            if (s2[0] != '[') s2 = '[' + s2 + ']';

            int val = matchVec(tokenize(s1), tokenize(s2));

            if (val != 0) { return val; }
        }
    }

    return v1.size() < v2.size() ? -1 : 0;
}

int compare(string s1, string s2) {
    if (s1[0] != '[') { s1 = '[' + s1 + ']'; }
    if (s2[0] != '[') s2 = '[' + s2 + ']';

    return matchVec(tokenize(s1), tokenize(s2));
}

int main(int argc, char* argv[]) {
    fstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line1, line2;

    int ans = 0;
    int idx = 1;

    while (getline(file, line1) && getline(file, line2)) {
        bool isMatch = compare(line1, line2) == -1;

        if (isMatch) { ans += idx; }

        getline(file, line1);

        idx++;
    }

    cout << ans << endl;
}
