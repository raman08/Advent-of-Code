#include <bits/stdc++.h>

#include <cstdlib>
#include <string>
using namespace std;

int main() {
    ifstream f;
    f.open("./input.txt");

    string line;
    int ans = 0;
    while (getline(f, line)) {
        cout << line << endl;

        string first = "";
        string second = "";
        string third = "";
        string fourth = "";

        bool a = false;
        bool b = false;
        bool c = false;

        for (auto it : line) {
            if (!a) {
                if (it != '-') {
                    first += it;
                } else {
                    a = true;
                    continue;
                }

            }

            else if (!b) {
                if (it != ',') {
                    second += it;
                } else {
                    b = true;
                    continue;
                }
            }

            else if (!c) {
                if (it != '-') {
                    third += it;
                } else {
                    c = true;
                    continue;
                }
            } else {
                fourth += it;
            }
        }

        int p = stoi(first);
        int q = stoi(second);
        int r = stoi(third);
        int s = stoi(fourth);

        if ((p >=r && p <= s) || (p <= r && q >= r)) {
            cout << p << " " << q << " " << r << " "
                 << s << endl;
            ans++;
        }
    }

    cout << ans;
}
