import nose

from solve import solve_part1
from solve import solve_part2
from solve import solve_part2_naive


def test_part1():
    nose.tools.assert_equal(solve_part1([1, 1, 1]), 3)
    nose.tools.assert_equal(solve_part1([1, 1, -2]), 0)
    nose.tools.assert_equal(solve_part1([-1, -2, -3]), -6)


def check_part2(numbers, solution):
    nose.tools.assert_equal(solve_part2_naive(numbers), solution)
    nose.tools.assert_equal(solve_part2(numbers), solution)


def test_part2_naive():
    check_part2([1, -1], (0, -1, 1, 0))
    check_part2([3, 3, 4, -2, -4], (10, 2, 1, 1))
    check_part2([-3, -3, -4, 2, 4], (-10, 2, 1, 1))
    check_part2([-6, 3, 8, 5, -6], (5, 2, 1, 2))
    check_part2([7, 7, -2, -7, -4], (14, 1, 2, 2))
    check_part2([50, 50, -99], (50, 0, 2, 49))
    check_part2([2, 2, 2, 7, -3], None)
