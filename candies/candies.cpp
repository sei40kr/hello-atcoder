#include <iostream>

// candies.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

#define FOR(i, a, b) for (int i = (a); i < (b); ++i)
#define RFOR(i, a, b) for (int i = (b)-1; i >= (a); --i)

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  int top[n];
  int bottom[n];
  REP(i, n) { cin >> top[i]; }
  REP(i, n) { cin >> bottom[i]; }

  FOR(i, 1, n) { top[i] += top[i - 1]; }
  RFOR(i, 0, n - 1) { bottom[i] += bottom[i + 1]; }

  int ans = 0;
  REP(i, n) { ans = max(ans, top[i] + bottom[i]); }

  cout << ans << endl;
}
