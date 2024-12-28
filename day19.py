import itertools


def split_list_on_string(lst, separator):
    return [
        list(group)
        for key, group in itertools.groupby(lst, lambda x: x != separator)
        if key
    ]


def is_matching_prefix(prefix, word):
    if len(prefix) > len(word):
        return False

    for i in range(len(prefix)):
        if word[i] != prefix[i]:
            return False

    return True


def dfs(dictionary, word, dp={}):
    if not word:
        return 1

    if word in dp:
        return dp[word]

    dp[word] = 0
    for d in dictionary:
        if is_matching_prefix(d, word):
            dp[word] += dfs(dictionary, word[len(d) :])

    return dp[word]


if __name__ == "__main__":
    import sys

    filename = sys.argv[1] if len(sys.argv) >= 2 else "in.test"
    file = open(filename, "r")
    [d, q] = split_list_on_string(file.read().split("\n"), "")
    d = d[0].split(", ")

    res = 0
    dp = {}
    for word in q:
        res += dfs(d, word, dp)

    print(res)
