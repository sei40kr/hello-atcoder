#include <bits/stdc++.h>

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  cin.tie(0);
  ios::sync_with_stdio(false);

  int n;
  cin >> n;

  int total = 0, min = 101;
  REP(i, n) {
    int s;
    cin >> s;

    if (s % 10 != 0 && s < min) {
      min = s;
    }

    total += s;
  }

  cout << (total % 10 == 0 ? (min == 101 ? 0 : total - min) : total) << endl;
}

