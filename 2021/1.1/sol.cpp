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
    bool start = true;
    int counter = 0, curr = 0, prev = 0;

    while (!in.eof()) {
        if (start) {
            start = false;
            in >> prev;
        }
        in >> curr;
        if (curr > prev) counter++;
        prev = curr;
    }

    cout << counter << endl;

    return 0;
}