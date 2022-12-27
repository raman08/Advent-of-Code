#include <bits/stdc++.h>

#include <cmath>

using namespace std;

unordered_map<char, long long int> mapping = {
    {'2', 2}, {'1', 1}, {'0', 0}, {'-', -1}, {'=', -2}};

string decimalToSnafu(long long int num) {
    string ans = "";
    while (num) {
        switch (num % 5) {
            case 0: {
                ans += '0';
                num /= 5;
                break;
            }
            case 1: {
                ans += '1';
                num /= 5;
                break;
            }
            case 2: {
                ans += '2';
                num /= 5;
                break;
            }
            case 3: {
                ans += '=';
                num /= 5;
                num++;
                break;
            }
            case 4: {
                ans += '-';
                num /= 5;
                num++;
                break;
            }
            default: cout << "Something went wrong!!"; exit(1);
        }
    }

    reverse(ans.begin(), ans.end());
    return ans;
}

long long int snafuToDecimal(string num) {
    long long int tmp = 0;
    reverse(num.begin(), num.end());
    for (unsigned long long int i = 0; i < num.size(); i++) {
        // unsigned long long int place = n.size() - i;

        // cout << i << " " << pow(5, i) << " " << n[i] << "
        // "
        //      << mapping[n[i]] << " "
        //      << pow(5, i) * mapping[n[i]] << endl;

        tmp += (pow(5, i) * mapping[num[i]]);
    }

    return tmp;
}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;
    vector<string> snafu;
    while (getline(file, line)) {
        // cout << line << endl;
        snafu.push_back(line);
    }
    cout << endl;

    unsigned long long int decimal_sum = 0;

    // snafu.clear();

    for (auto n : snafu) {
        // string n = " -02=1= ";
        auto num = snafuToDecimal(n);
        decimal_sum += (num);
        cout << n << " -> " << num << endl;
    }

    // vector<long long int> numbers = {
    //     1, 2,  3,  4,  5,    6,     7,        8,
    //     9, 10, 15, 20, 2022, 12345, 314159265};
    //
    // for (auto num : numbers) {
    //     cout << num << " -> " << decimalToSnafu(num) << endl;
    // }
    // cout << endl;

    auto ans = decimalToSnafu(decimal_sum);

    cout << decimal_sum << " " << ans << endl;
}
