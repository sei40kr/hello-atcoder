#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0);
  ios::sync_with_stdio(false);

  string s;
  cin >> s;

  bool found['z' - 'a' + 1] = {false};

  for (char &c : s) {
    int i = c - 'a';

    if (found[i]) {
      cout << "no" << endl;
      return 0;
    }

    found[i] = true;
  }

  cout << "yes" << endl;
}

