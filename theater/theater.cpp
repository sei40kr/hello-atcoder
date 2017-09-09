#include <bits/stdc++.h>

// theater.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  int ans = 0;
  REP(i, n) {
    int l, r;
    cin >> l >> r;
    ans += r - l + 1;
  }

  cout << ans << endl;
}
