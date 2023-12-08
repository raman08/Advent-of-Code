#include <bits/stdc++.h>

#include <memory>
#include <ratio>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

using namespace std;

struct File {
    string name;
    long long size;
};

struct Directory {
    string name;
    long long size;

    vector<unique_ptr<Directory>> dirs;
    vector<unique_ptr<File>> files;

    Directory* parent;
};

void traverse(Directory* node, int level) {
    for (int i = 0; i < level; i++) { cout << "  "; }

    cout << "- " << node->name << "(dir, size=" << node->size << ')' << endl;

    for (auto& dir : node->dirs) { traverse(dir.get(), level + 1); }

    for (auto& file : node->files) {
        for (int i = 0; i < level + 1; i++) { cout << "  "; }

        cout << "- " << file->name << "(file, size=" << file->size << ')' << endl;
    }
}

long long updateSize(Directory* node) {
    long long sz = 0;
    for (auto& dir : node->dirs) { sz += updateSize(dir.get()); }

    for (auto& file : node->files) { sz += file->size; }

    return node->size = sz;
}

pair<long long, long long> countDirs(Directory* node, long long threshold) {
    long long sum = 0;
    long long count = 0;

    for (auto& dir : node->dirs) {
        if (dir->size <= threshold) {
            count++;
            sum += dir->size;
        }

        auto ans = countDirs(dir.get(), threshold);

        count += ans.first;
        sum += ans.second;
    }

    return {count, sum};
}

int main() {
    ifstream f;
    // f.open("./input-ex.txt");
    f.open("./input.txt");

    auto fs = make_unique<Directory>();

    fs->name = "/";
    fs->parent = fs.get();

    string line;

    auto current = fs.get();

    bool skip = false;

    while (skip || getline(f, line)) {
        cout << line << endl;

        skip = false;

        if (line[0] == '$') {
            int i = line.find(' ', 2);
            string cmd = line.substr(2, i - 2);

            if (cmd == "cd") {
                string name = line.substr(i + 1, line.size() - i - 1);

                if (name == "..") {
                    current = current->parent;
                    cout << "Current dir is " << current->name << endl;
                }

                else if (name == "/") {
                    current = fs.get();
                    cout << "Current dir is " << current->name << endl;
                }

                else {
                    cout << "Name is " << name;
                    bool found = false;

                    for (auto& dir : current->dirs) {
                        cout << current->name << " *********** " << dir->name << endl;
                        if (dir->name == name) {
                            found = true;
                            current = dir.get();
                            cout << "Current dir is " << current->name << endl;
                            break;
                        }
                    }

                    if (!found) {
                        cout << "Adding dir " << name << " with parent " << current->name << endl;
                        current->dirs.emplace_back();
                        current->dirs.back()->name = name;
                        current->dirs.back()->parent = current;
                        current = current->dirs.back().get();
                        cout << "Current dir is " << current->name << endl;
                    }
                }
            } else if (cmd == "ls") {
                while (getline(f, line)) {
                    cout << line << endl;

                    if (line[0] == 'd') {
                        string name = line.substr(4);

                        bool found = false;
                        for (auto& dir : current->dirs) {
                            if (dir->name == name) {
                                found = true;
                                cout << "Found dir " << name << endl;
                            }
                        }

                        if (!found) {
                            cout << "Adding dir " << name << " with parent " << current->name
                                 << endl;

                            current->dirs.push_back(make_unique<Directory>());
                            current->dirs.back()->name = name;
                            current->dirs.back()->parent = current;
                        }
                    }

                    else if (line[0] != '$') {
                        int i = line.find(' ');

                        string size = line.substr(0, i);
                        string name = line.substr(i + 1, line.size() - i - 1);

                        cout << "Adding file " << name << " of size " << size << " with parent "
                             << current->name << endl;

                        current->files.push_back(make_unique<File>());
                        current->files.back()->size = stoi(size);
                        current->files.back()->name = name;

                    } else {
                        skip = true;
                        break;
                    }
                }
            }
        }
    }

    traverse(fs.get(), 0);

    updateSize(fs.get());

    traverse(fs.get(), 0);

    long long threshold = 100000;

    auto ans = countDirs(fs.get(), threshold);

    if (fs->size <= threshold) {
        ans.first++;
        ans.second += fs->size;
    }

    cout << ans.first << " " << ans.second << endl;
}
