#include <bits/stdc++.h>

// star-in-parentheses.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  string str;
  cin >> str;

  int l = 0, r = 0;
  bool b = false;
  for (const char c : str) {
    if (c == '*') {
      b = true;
      continue;
    }
    int *x = b ? &r : &l;
    *x = max(0, *x + (b ^ (c == '(') ? 1 : -1));
  }

  cout << min(r, l) << endl;
}
