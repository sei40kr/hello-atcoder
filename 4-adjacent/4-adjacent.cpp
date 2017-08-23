#include <bits/stdc++.h>

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(0);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;
  int as[n];
  REP(i, n) {
    cin >> as[i];
  }

  int xs[] = {0, 0, 0};
  REP(i, n) {
    int a = as[i];
    if (a % 4 == 0) {
      ++xs[2];
    } else if (a % 2 == 0) {
      ++xs[1];
    } else {
      ++xs[0];
    }
  }

  int d = xs[0] - xs[2];
  bool ans = d <= 0 || (d == 1 && xs[1] == 0);
  cout << (ans ? "Yes" : "No") << endl;
}

