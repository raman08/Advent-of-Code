#include <bits/stdc++.h>

#include <string>
#include <unordered_set>

using namespace std;

struct Point {
    int x;
    int y;

    Point() {
        this->x = 0;
        this->y = 0;
    }

    Point(int x, int y) {
        this->x = x;
        this->y = y;
    }
};

int main(int argc, char* argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    string line;

    Point* H = new Point(0, 0);
    Point* T = new Point(0, 0);

    unordered_set<int> st;

    while (getline(f, line)) {
        cout << line << endl;

        int steps = stoi(line.substr(2));
        char dir = line[0];

        for (int i = 0; i < steps; i++) {
            switch (dir) {
                case 'R': {
                    H->x++;
                    break;
                };
                case 'U': {
                    H->y++;
                    break;
                };
                case 'L': {
                    H->x--;
                    break;
                };
                case 'D': {
                    H->y--;
                    break;
                };
            }
            cout << "H: " << H->x << " " << H->y << endl;

            int xg = H->x - T->x;
            int yg = H->y - T->y;

            cout << "Diff is:" << xg << " " << yg << endl;
            if (abs(xg) == 2 && abs(yg) == 0) {
                T->x += xg / 2;
            } else if (abs(xg) == 2 && abs(yg) == 1) {
                T->x += xg / 2;
                T->y += yg;
            } else if (abs(xg) == 0 && abs(yg) == 2) {
                T->y += yg / 2;
            } else if (abs(xg) == 1 && abs(yg) == 2) {
                T->x += xg;
                T->y += yg / 2;
            }

            cout << "T: " << T->x << " " << T->y << endl;
            cout << endl;
            st.insert(T->x * 100000 + T->y);
        }
        cout << endl;
    }

    cout << st.size();
}
