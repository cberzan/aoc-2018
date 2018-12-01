#!/usr/bin/env python

from collections import defaultdict
import sys


def solve_part1(numbers):
    return sum(numbers)


def solve_part2_naive(numbers):
    """
    Returns the solution as (value, i, j, k).

    Interpretation:
        value == sum(numbers[:i-1])
              == sum(numbers[:j-1]) + k * sum(numbers).

    If there is no solution, returns None.
    """
    cumsum_to_i = {0: -1}
    cumsum = 0
    for j, number in enumerate(numbers):
        cumsum += number
        if cumsum in cumsum_to_i:
            return (cumsum, cumsum_to_i[cumsum], j, 0)
        cumsum_to_i[cumsum] = j

    # If there is a solution, then it must involve one of the values seen in
    # the first pass. Consider: sum(numbers[:i-1]) + ki * sum(numbers) ==
    # sum(numbers[:j-1]) + kj * sum(numbers). Then we can find a solution with
    # ki = 0 by subtracting a multiple of sum(numbers) from both sides.
    # Therefore we do not need to add more entries to cumsum_to_i for k > 0.
    # When all the values we see have gone outside the range for k = 0, then no
    # solution is possible.
    range_min = min(cumsum_to_i.iterkeys())
    range_max = max(cumsum_to_i.iterkeys())

    k = 1
    while True:
        found_value_in_range = False
        for j, number in enumerate(numbers):
            cumsum += number
            if cumsum in cumsum_to_i:
                return (cumsum, cumsum_to_i[cumsum], j, k)
            if range_min <= cumsum <= range_max:
                found_value_in_range = True
        if not found_value_in_range:
            return None
        k += 1


def soln_less_than(soln_a, soln_b):
    """
    Returns True iff soln_a is preferable to soln_b.
    """
    assert soln_a is not None
    if soln_b is None:
        return True

    va, ia, ja, ka = soln_a
    vb, ib, jb, kb = soln_b
    return (ka, ja, ia) < (kb, jb, ib)


def solve_part2(numbers):
    """
    As above, but using a fancy O(n log n) solution.
    """
    seq_sum = sum(numbers)

    # Case 1: We have k = 0. Get the solution in O(n).
    if seq_sum == 0:
        cumsum_to_i = {0: -1}
        cumsum = 0
        for j, number in enumerate(numbers):
            cumsum += number
            if cumsum in cumsum_to_i:
                return (cumsum, cumsum_to_i[cumsum], j, 0)
            cumsum_to_i[cumsum] = j
        assert False, "should not reach here"

    # Case 2: We have k > 0. Get the solution in O(n log n).
    else:
        # Group the cumsum values by their module when divided by seq_sum.
        mod_to_indices = defaultdict(list)
        cumsum = 0
        cumsums = []
        for j, number in enumerate(numbers):
            cumsum += number
            cumsums.append(cumsum)
            mod_to_indices[cumsum % seq_sum].append(j)

        # If i and j form a solution, they must be in the same modulo group.
        best_soln = None
        for mod, indices in mod_to_indices.iteritems():
            indices.sort(key=lambda i: cumsums[i] / seq_sum)
            for t in xrange(1, len(indices)):
                j = indices[t - 1]
                i = indices[t]
                div_j = cumsums[j] / seq_sum
                div_i = cumsums[i] / seq_sum
                assert div_j <= div_i
                k = div_i - div_j
                soln = (cumsums[i], i, j, k)
                if soln_less_than(soln, best_soln):
                    best_soln = soln
        return best_soln


if __name__ == "__main__":
    numbers = []
    for line in sys.stdin:
        numbers.append(int(line))
    print "part 1:", solve_part1(numbers)
    print "part 2 naive:", solve_part2_naive(numbers)
    print "part 2 fast:", solve_part2(numbers)
