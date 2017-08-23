#include <bits/stdc++.h>

using namespace std;

int main() {
  cin.tie(0);
  ios::sync_with_stdio(false);

  string s;
  cin >> s;
  size_t len = s.length();
  cout << s[0] << len - 2 << s[len - 1] << endl;
}
