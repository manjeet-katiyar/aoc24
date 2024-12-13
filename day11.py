f = open("in", "r")

dp = {}
def dfs(num, rem):
    if rem == 0:
        return 1


    if (num, rem) in dp:
        return dp[(num, rem)]

    if num == 0:
        dp[(num, rem)] = dfs(1, rem - 1)
        return dp[(num, rem)]

    if len(str(num)) % 2 == 0:
        num = str(num)
        l = len(num)
        dp[(int(num), rem)] = dfs(int(num[0:l//2]), rem - 1) + dfs(int(num[l//2:]), rem - 1)
        return dp[(int(num), rem)]


    dp[(num, rem)] = dfs(num * 2024, rem - 1)
    return dp[(num, rem)]


res1 = 0
for i in f.read().split(' '):
    res1 += dfs(int(i), 75)

print(res1, len(dp))

