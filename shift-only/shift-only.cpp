#include <bits/stdc++.h>

// shift-only.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2017 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;

  int ans = INT_MAX;
  REP(i, n) {
    int x;
    cin >> x;

    int cnt = 0;

    while (x % 2 == 0) {
      x /= 2;
      ++cnt;
    }

    ans = min(ans, cnt);
  }

  cout << ans << endl;
}
