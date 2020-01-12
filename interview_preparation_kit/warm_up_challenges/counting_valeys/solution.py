#https://www.hackerrank.com/challenges/counting-valleys/problem?h_l=interview&playlist_slugs%5B%5D=interview-preparation-kit&playlist_slugs%5B%5D=warmup
def countingValleys(n, s):
    """
    >>> countingValleys(8, 'UDDDUDUU')
    1
    >>> countingValleys(12, 'DDUUDDUDUUUD')
    2
    """
    current_level = 0
    counts = 0
    path = map(lambda k: 1 if k == 'U' else -1, s)
    for p in path:
        last_level = current_level
        current_level += p
        if last_level == -1 and current_level == 0:
            counts += 1

    return counts
