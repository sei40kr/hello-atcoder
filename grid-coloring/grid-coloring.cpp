#include <bits/stdc++.h>

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(0);
  ios::sync_with_stdio(false);

  int h, w, n;
  cin >> h >> w >> n;
  int as[n];
  REP(i, n) {
    cin >> as[i];
  }

  int grid[h][w];
  // The number of colorized cells.
  int cnt = 0;
  REP(i, n) {
    int a = as[i];

    while (--a >= 0) {
      int x = cnt / h;
      int y = cnt % h;
      if (x % 2 == 1) {
        y = h - y - 1;
      }

      grid[y][x] = i + 1;

      ++cnt;
    }
  }

  REP(y, h) {
    REP(x, w) {
      cout << (x == 0 ? "" : " ") << grid[y][x];
    }

    cout << endl;
  }
}
