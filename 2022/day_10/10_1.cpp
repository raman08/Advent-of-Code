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
    long long cycles = 1;

    long long ans = 0;
    while (getline(f, line)) {
        // cout << line << endl;

        if (line[0] == 'n') {
            cycles++;

            if (checkCycle(cycles)) {
                cout << cycles << " " << cycles * X << endl;
                ans += (cycles * X);
            }

        } else if (line[0] == 'a') {
            int v = stoi(line.substr(4));

            cycles++;

            if (checkCycle(cycles)) {
                cout << cycles << " " << cycles * X << endl;
                ans += (cycles * X);
            }

            cycles++;
            X += v;

            if (checkCycle(cycles)) {
                cout << cycles << " " << cycles * X << endl;
                ans += (cycles * X);
            }
        }

        // if (checkCycle(cycles)) {
        //     cout << cycles << " " << cycles * X << endl;
        //     ans += (cycles * X);
        // }
    }

    cout << ans << endl;
}
