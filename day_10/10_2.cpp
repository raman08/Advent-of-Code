#include <bits/stdc++.h>

#include <string>

using namespace std;

bool checkCycle(long long cycles) {
    return cycles == 20 || cycles == 60 || cycles == 100 || cycles == 140 || cycles == 180 ||
           cycles == 220;
}

int main(int argc, char *argv[]) {
    ifstream f;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    f.open(fileName);

    string line;
    long long X = 1;
    long long cycles = 0;

    long long ans = 0;

    vector<vector<char>> CRT(6, vector<char>(40, '-'));
    vector<bool> sprite(40, false);

    sprite[0] = sprite[1] = sprite[2] = true;

    while (getline(f, line)) {
        cout << line << endl;

        if (line[0] == 'n') {
            cycles++;

            int cm = (cycles - 1) % 240;
            int cx = (cm) / 40;
            int cy = (cm) % 40;

            cout << "Drawing at " << cx << " " << cy << endl;
            if (cy == X || cy == X - 1 || cy == X + 1) {
                CRT[cx][cy] = '#';
            } else {
                CRT[cx][cy] = '.';
            }

        } else if (line[0] == 'a') {
            int v = stoi(line.substr(4));

            cycles++;

            int cm = (cycles - 1) % 240;
            int cx = (cm) / 40;
            int cy = (cm) % 40;

            cout << "Drawing at " << cx << " " << cy << endl;
            if (cy == X || cy == X - 1 || cy == X + 1) {
                CRT[cx][cy] = '#';
            } else {
                CRT[cx][cy] = '.';
            }

            cycles++;

            cm = (cycles - 1) % 240;
            cx = (cm) / 40;
            cy = (cm) % 40;

            cout << "Drawing at " << cx << " " << cy << endl;
            if (cy == X || cy == X - 1 || cy == X + 1) {
                CRT[cx][cy] = '#';
            } else {
                CRT[cx][cy] = '.';
            }

            X += v;
        }

        for (auto row : CRT) {
            for (auto col : row) { cout << col << " "; }
            cout << endl;
        }
        cout << endl;
        cout << endl;
    }

    for (auto row : CRT) {
        for (auto col : row) { cout << col; }
        cout << endl;
    }
    cout << endl;
    cout << endl;
    // cout << ans << endl;
}
