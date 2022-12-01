package solution

import (
	"testing"
)

type PalindromeTest struct{ T *testing.T }

func (t PalindromeTest) TestPalindromePairs() {
	println("testing solution")
	/*
	   utils.AssertEqualArray(PalindromePairs([]string{"bat", "tab", "cat"}), [][]int{{0, 1}, {1, 0}})
	   utils.AssertEqualArray(PalindromePairs([]string{"dog", "cow", "tap", "god", "pat"}), [][]int{{0, 3}, {2, 4}, {3, 0}, {4, 2}})
	   utils.AssertEqualArray(PalindromePairs([]string{"abcd", "dcba", "lls", "s", "sssll"}), [][]int{{0, 1}, {1, 0}, {2, 4}, {3, 2}})
	*/
}

func PalindromeTests(t *testing.T) {
	x := PalindromeTest{T: t}
	x.TestPalindromePairs()
}
