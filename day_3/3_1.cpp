#include <bits/stdc++.h>

#include <unordered_map>
#include <unordered_set>

using namespace std;

int main() {
    ifstream f;
    f.open("./input.txt");

    string line;
    int ans = 0;
    while (getline(f, line)) {
        cout << line << endl;

        unordered_set<char> st;

        for (int i = 0; i < line.size() / 2; i++) {
            st.insert(line[i]);
        }

        for (int i = line.size() / 2; i < line.size();
             i++) {
            char ch = line[i];
            if (st.find(ch) != st.end()) {
                cout << ch << " ";
                if (ch >= 65 && ch <= 92) {
                    ch = ch + 32;

                    ans += ch - 'a' + 27;
                    cout << ch - 'a' + 27 << endl;

                    break;
                }

                ans += ch - 'a' + 1;
                cout << ch - 'a' + 1 << endl;
                break;
            }
        }
    }

    cout << ans << endl;
}
