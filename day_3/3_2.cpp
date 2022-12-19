#include <bits/stdc++.h>

#include <unordered_set>

using namespace std;

int main() {
    ifstream f;
    f.open("./input.txt");

    string line;
    int ans = 0;

    vector<string> grp;

    while (getline(f, line)) {
        cout << line << endl;

        grp.push_back(line);

        cout << grp.size() << endl;

        unordered_set<char> st1;
        unordered_set<char> st2;
        if (grp.size() == 3) {
            for (auto it : grp[0]) { st1.insert(it); }

            for (auto it : grp[1]) { st2.insert(it); }

            for (auto ch : grp[2]) {
                if (st1.find(ch) != st1.end() && st2.find(ch) != st2.end()) {
                    cout << "Common: " << ch << endl;
                    if (ch >= 65 && ch <= 92) {
                        ch = ch + 32;

                        ans += ch - 'a' + 27;
                        // cout << ch - 'a' + 27 <<
                        // endl;

                        break;
                    }

                    ans += ch - 'a' + 1;
                    // cout << ch - 'a' + 1 << endl;
                    break;
                }
            }

            grp.clear();
        }
    }

    cout << ans;
}
