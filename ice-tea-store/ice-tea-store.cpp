#include <bits/stdc++.h>

// ice-tea-store.cpp
// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  vector<pair<int, int>> ps;
  long n;
  REP(i, 4) {
    int v, p;
    v = static_cast<int>(pow(2, i));
    cin >> p;
    p *= 8;
    ps.emplace_back(p / v, v);
  }
  cin >> n;
  n *= 4;

  sort(ps.begin(), ps.end());

  long ans = 0;
  while (0 < n) {
    for (const auto &pair : ps) {
      int p = pair.first, v = pair.second;
      if (v <= n) {
        long m = n / v;
        n %= v;
        ans += (long) p * v * m;
        break;
      }
    }
  }

  ans /= 8;
  cout << ans << endl;
}

