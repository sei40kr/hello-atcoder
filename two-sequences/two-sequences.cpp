#include <algorithm>
#include <iostream>

#define REP(i, n) for (int i = 0; i < (n); i++)

// two-sequences.cpp --- Two Sequences
// author: Seong Yong-ju <sei40kr@gmail.com>

using namespace std;

int main() {
  int n;
  cin >> n;

  int as[n], bs[n];
  REP(i, n) { cin >> as[i]; }
  REP(i, n) { cin >> bs[i]; }

  int ans = 0;

  REP(k, 29) {
    int t = 1 << k;

    int ys[n];
    REP(i, n) { ys[i] = bs[i] % (t << 1); }
    sort(ys, ys + n);

    REP(i, n) {
      int x = as[i] % (t << 1);

      int *p1 = lower_bound(ys, ys + n, t - x);
      int *p2 = lower_bound(p1, ys + n, (t << 1) - x);
      int *p3 = lower_bound(p2, ys + n, (t << 1) + t - x);

      ans ^= (((p2 - p1) + ((ys + n) - p3)) & 1) << k;
    }
  }

  cout << ans << endl;
}
