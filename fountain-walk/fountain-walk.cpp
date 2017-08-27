#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

#define P pair<int, int>

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  P start, dest;
  int n;
  cin >> start.first >> start.second >> dest.first >> dest.second >> n;
  bool invert = false;
  if (dest.first < start.first) {
    swap(start, dest);
  }
  if (dest.second < start.second) {
    invert = true;
    start.second *= -1;
    dest.second *= -1;
  }
  set<pair<int, int>> ps;
  REP(i, n) {
    int x, y;
    cin >> x >> y;
    if (invert) {
      y *= -1;
    }
    if (start.first <= x && x <= dest.first && start.second <= y &&
        y <= dest.second) {
      ps.emplace(x, y);
    }
  }

  P vec = make_pair(dest.first - start.first, dest.second - start.second);

  size_t l = ps.size();
  int dp[l];
  fill(dp, dp + l, INT_MAX);
  for_each(ps.begin(), ps.end(), [l, &dp](const auto &p) {
    *lower_bound(dp, dp + l, p.second) = p.second;
  });
  long k = lower_bound(dp, dp + l, INT_MAX) - dp;

  double ans = (double)100 * (vec.first + vec.second) + (5 * M_PI - 20) * k;
  if (min(vec.first, vec.second) < k) {
    ans += 5 * M_PI;
  }

  cout << fixed << setprecision(19) << ans << endl;
}
