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
    string x;
    int y, depth = 0, pos = 0, aim = 0;

    while (!in.eof()) {
        in >> x >> y;
        switch (x[0]) {
            case 'f':
                pos += y;
                depth += y * aim;
                break;
            case 'u':
                aim -= y;
                break;
            case 'd':
                aim += y;
                break;
            default:
                break;
        }
    }

    printf("Pos: %d\nDepth: %d\nMultiplied: %d\n", pos, depth, pos * depth);

    return 0;
}