#include <bits/stdc++.h>

// reverse-and-compare.cpp
// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  string str;
  cin >> str;
  size_t l = str.length();
  int cs['z' - 'a' + 1] = {};

  for (const char &c : str) {
    ++cs[c - 'a'];
  }

  long ans = 0;
  for (const char &c : str) {
    ans += l - cs[c - 'a'];
  }
  ans /= 2;
  ++ans;

  cout << ans << endl;
}

