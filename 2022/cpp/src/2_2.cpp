
#include <bits/stdc++.h>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <queue>
#include <math.h>
#include <map>
#include <set>
#include <unordered_set>
#include <stack>
#include <list>
#include <bitset>
#include <utility>
#include <algorithm>
#include <iomanip>
#include <cstdio>
#include <cstring>
#include <numeric>

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
ll gcd(ll a, ll b) { return b ? gcd(b, a % b) : a; }
ll lcm(ll a, ll b) { return a * (b / gcd(a, b)); }

auto inside = [](int row, int col, int r, int c)
{
    return 0 <= row && 0 <= col && row < r && col < c;
};

int main()
{
    // Read file
    ifstream in("../data/in/2.in");

    int sum = 0;

    for (string line; getline(in, line);)
    {

        if (line == "A X")
            sum += 3;
        else if (line == "A Y")
            sum += 4;
        else if (line == "A Z")
            sum += 8;
        else if (line == "B X")
            sum += 1;
        else if (line == "B Y")
            sum += 5;
        else if (line == "B Z")
            sum += 9;
        else if (line == "C X")
            sum += 2;
        else if (line == "C Y")
            sum += 6;
        else if (line == "C Z")
            sum += 7;
    }

    cout << sum << endl;
    return 0;
}