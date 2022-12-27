#include <bits/stdc++.h>

using namespace std;

enum YellType { NUMBER, SOLVE };

struct Monkey {
    Monkey() {}

    Monkey(int id, std::string name, YellType type,
           long long val, std::string dep1, std::string dep2,
           char operation, bool calculated)
        : id(id),
          name(std::move(name)),
          type(type),
          val(val),
          dep1(std::move(dep1)),
          dep2(std::move(dep2)),
          operation(operation),
          calculated(calculated) {}

    int id;
    string name;
    YellType type;
    long long val;
    string dep1;
    string dep2;
    char operation;
    bool calculated;
};

long long findResult(long long op1, long long op2, char op) {
    if (op == '+') { return op1 + op2; }

    if (op == '-') { return op1 - op2; }

    if (op == '*') { return op1 * op2; }

    if (op == '/') { return op1 / op2; }
}

int main(int argc, char* argv[]) {
    ifstream file;

    string fileName = "./input-ex.txt";
    if (argc > 1) fileName = "./input.txt";

    file.open(fileName);

    string line;

    vector<Monkey> monkeys;
    unordered_map<string, int> monkeysMap;

    int idx = 0;
    while (getline(file, line)) {
        cout << line << endl;

        auto monkey = Monkey();

        monkey.id = idx++;
        monkey.name = line.substr(0, 4);

        if (line[6] >= '0' && line[6] <= '9') {
            monkey.type = NUMBER;
            monkey.val = stoll(line.substr(6, line.size() - 6));
        } else {
            monkey.type = SOLVE;

            auto start = 6;
            auto end = line.find(' ', start);
            monkey.dep1 = line.substr(start, end - start);

            start = end + 1;
            monkey.operation = line[start];

            start = start + 2;
            monkey.dep2 =
                line.substr(start, line.size() - start);
        }

        monkey.calculated = false;
        monkeys.push_back(monkey);
        monkeysMap[monkey.name] = monkey.id;
    }
    cout << endl;

    long long count = 0;

    // for (auto monkey : monkeys) {
    //     cout << "Monkey: " << monkey.name << " have dep on "
    //          << monkey.dep1 << " "
    //          << " and " << monkey.dep2 << " " << endl;
    // cout << endl;
    // }

    while (true) {
        // cout << count << endl;
        for (auto& monkey : monkeys) {
            if (monkey.calculated) { continue; }

            // cout << "Monkey: " << monkey.name << " "
            //      << monkey.type << endl;
            if (monkey.type == SOLVE) {
                cout << "Monkey: " << monkey.name
                     << " have dep on " << monkey.dep1 << " "
                     << " and " << monkey.dep2 << " " << endl;

                if (!monkeys[monkeysMap[monkey.dep1]]
                         .calculated) {
                    continue;
                }

                if (!monkeys[monkeysMap[monkey.dep2]]
                         .calculated) {
                    continue;
                }

                monkey.val = findResult(
                    monkeys[monkeysMap[monkey.dep1]].val,
                    monkeys[monkeysMap[monkey.dep2]].val,
                    monkey.operation);

                monkey.calculated = true;

                cout << "For: " << monkey.name << " "
                     << monkeys[monkeysMap[monkey.dep1]].val
                     << " " << monkey.operation << " "
                     << monkeys[monkeysMap[monkey.dep2]].val
                     << " = " << monkey.val << endl;

                if (monkey.name == "root") {
                    cout << "Found Root: ";
                    cout << monkey.val << endl;
                    return 0;
                }

            } else {
                cout << "Solved for: " << monkey.name << " "
                     << monkey.val << endl;
                monkey.calculated = true;
            }
        }
        cout << endl;

        count++;
    }
}
