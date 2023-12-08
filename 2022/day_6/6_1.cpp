#include <bits/stdc++.h>

#include <list>
#include <unordered_set>

using namespace std;

int main() {
    ifstream f;
    // f.open("./input-ex.txt");
    f.open("./input.txt");

    string line;
    while (getline(f, line)) {
        cout << line << endl;

        bool found = false;
        int idx = 0;
        list<char> st;

        while (!found) {
            while (st.size() < 3) {
                st.push_back(line[idx++]);
            }

            unordered_set<char> ss;
            for (auto it : st) { ss.insert(it); }

            // for (auto it : st) { cout << it << " "; }
            // cout << endl;

            if (ss.size() != 3) {
                // cout << "Fail at set !! " << endl;
                // cout << "Removing " << st.front()
                //      << endl;
                st.pop_front();
                continue;
            }

            char ch = line[idx];
            // cout << "Checking ch: " << ch << endl;

            bool sameChar = false;
            for (auto it : st) {
                if (it == ch) {
                    sameChar = true;
                    break;
                }
            }

            if (sameChar) {
                // cout << "Removing " << st.front()
                //      << endl;
                st.pop_front();

                // idx++;
            } else {
                found = true;
            }
        }

        cout << idx + 1 << endl;
    }
}
