#include <bits/stdc++.h>

using namespace std;

#define all(x) (x).begin(), (x).end()
#define mp make_pair
#define fi first
#define se second
#define pb push_back
#define eb emplace_back
#define forn(i, n) for (int i = 0; i < (int)n; ++i)
#define ford(i, n) for (int i = (int)n - 1; i >= 0; --i)

typedef long long ll;
typedef vector<int> vi;
typedef vector<vi> vvi;
typedef pair<int, int> ii;
typedef vector<ii> vii;
typedef vector<vii> vvii;

const ll mod = 1000000007;
int gcd(int a, int b) { return b ? gcd(b, a % b) : a; }
ll lcm(ll a, ll b) { return a * (b / gcd(a, b)); }

int main() {
    ifstream in("in.txt");
    vii counters(12, {0, 0});
    int res = 0, mask = (1 << 12) - 1;
    string tmp;
    while (!in.eof()) {
        in >> tmp;
        for (int i = 0; i < 12; ++i) tmp[i] == '0' ? counters[i].fi++ : counters[i].se++;
    }

    for (int i = 0; i < 12; ++i) {
        if (counters[i].se > counters[i].fi) res |= (1 << 11 - i);
    }

    int gamma = res, epsilon = (~res) & mask;

    printf("Gamma: %d\nEpsilon: %d\nResult: %d\n", gamma, epsilon, gamma * epsilon);

    return 0;
}