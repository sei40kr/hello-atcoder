#include <bits/stdc++.h>

// katana-thrower.cpp
// author: Seong Yong-ju ( @sei40kr )
// Copyright (c) 2018 Seong Yong-ju

#define REP(i, n) for (int i = 0; i < (n); ++i)
#define RREP(i, n) for (int i = (n)-1; i >= 0; --i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int N, H;
  cin >> N >> H;

  int as[N], bs[N];
  REP(i, N) { cin >> as[i] >> bs[i]; }

  int a_max = *max_element(as, as + N);
  sort(bs, bs + N);

  int ans = 0;

  RREP(i, N) {
    if (H <= 0) {
      break;
    }

    int b = bs[i];
    if (b < a_max) {
      break;
    }

    ++ans;
    H -= b;
  }

  if (0 < H) {
    ans += ceil(static_cast<double>(H) / a_max);
  }

  cout << ans << endl;
}
