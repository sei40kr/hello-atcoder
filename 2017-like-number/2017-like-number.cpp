#include <iostream>

#define FOR(i, a, b) for (int i = (a); i < (b); i++)
#define REP(i, n) for (int i = 0; i < (n); i++)

// 2017-like-number.cpp --- 2017-like Number
// author: Seong Yong-ju <sei40kr@gmail.com>

bool fs[100001];
int imos[100001];

using namespace std;

int main() {
  FOR(i, 2, 100001) {
    if (!fs[i]) {
      for (int j = i << 1; j < 100001; j += i) {
        fs[j] = true;
      }
    }
  }

  for (int i = 3; i < 100001; i += 2) {
    if (!fs[i] && !fs[(i + 2) >> 1]) {
      ++imos[i];
    }
  }

  FOR(i, 3, 100001) { imos[i] += imos[i - 1]; }

  int n;
  cin >> n;

  REP(i, n) {
    int l, r;
    cin >> l >> r;

    cout << (imos[r] - imos[l - 1]) << endl;
  }
}
