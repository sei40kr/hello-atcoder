#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

#define REP(i, n) for (int i = 0; i < (n); ++i)

using namespace std;

int main() {
  int n;
  string s1, s2;
  cin >> n >> s1 >> s2;

  bool last_vert = false;
  long ans = 1;
  REP(i, n) {
    char c1 = s1[i], c2 = s2[i];
    bool vert = c1 == c2;

    ans *= i == 0 ? (vert ? 3 : 6) : (last_vert ? 2 : (vert ? 1 : 3));
    ans %= 1000000007;

    if (!vert) {
      ++i;
    }
    last_vert = vert;
  }

  cout << ans << endl;
}
