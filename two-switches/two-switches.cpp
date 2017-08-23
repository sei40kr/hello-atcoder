#include <bits/stdc++.h>

// author: Seong Yong-ju ( @sei40kr )

#define MIN(a, b) (a > b ? b : a)
#define MAX(a, b) (a > b ? a : b)

using namespace std;

int main() {
  cin.tie(nullptr);
  ios::sync_with_stdio(false);

  int a, b, c, d;
  cin >> a >> b >> c >> d;
  int ans = MIN(b, d) - MAX(a, c);
  cout << (MAX(0, ans)) << endl;
}
