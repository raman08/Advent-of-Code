#include "bits/stdc++.h"

using namespace std;

struct Number {
    int64_t index;
    int64_t val;
};

bool operator==(const Number& a, const Number& b) {
    return std::tuple<size_t, size_t>(a.index, a.val) ==
           std::tuple<size_t, size_t>(b.index, b.val);
}

int64_t mod(int64_t a, int64_t b) { return (a % b + b) % b; }

void move(vector<Number>& vec, Number& num) {
    auto it = std::find(vec.begin(), vec.end(), num);
    auto pos = mod(std::distance(vec.begin(), it) + num.val,
                   vec.size() - 1ULL);

    vec.erase(it);
    vec.insert(vec.begin() + pos, num);
}

auto coordinate(const vector<Number>& numbers, int64_t zero_pos,
                int64_t v) {
    return numbers[(v + zero_pos) % numbers.size()].val;
};

auto coordinate_sum(const vector<Number>& numbers) {
    auto zero_pos = std::distance(
        numbers.begin(),
        std::find_if(numbers.begin(), numbers.end(),
                     [](auto& num) { return num.val == 0; }));

    return coordinate(numbers, zero_pos, 1000) +
           coordinate(numbers, zero_pos, 2000) +
           coordinate(numbers, zero_pos, 3000);
}

vector<Number> mix(vector<Number>& coded,
                   vector<Number>& orig) {
    vector<Number> decoded = coded;

    for (auto& num : orig) { move(decoded, num); }

    return decoded;
}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    vector<Number> encryption;

    int idx = 0;
    while (getline(file, line)) {
        // cout << line << endl;
        encryption.push_back({idx++, stoi(line)});
    }
    cout << endl;

    auto key_coded = encryption;

    int key = 811589153;
    // int key = 1;
    int mixing = 10;

    for (int i = 0; i < key_coded.size(); i++) {
        key_coded[i].val *= key;
    }

    auto decryption = key_coded;

    for (int i = 0; i < mixing; i++) {
        cout << "Mixing " << i + 1 << endl;
        decryption = mix(decryption, key_coded);
    }

    for (auto it : decryption) { cout << it.val << " "; }
    cout << endl;

    cout << coordinate_sum(decryption);

    cout << endl;
}
