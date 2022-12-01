package solution

import (
	"fmt"
)

/**
* Given a list of unique words. Find all pairs of distinct indices (i, j) in the given list so that the
concatenation of the two words, i.e. words[i] + words[j] is a palindrome.
*
*/

type combined_words struct {
	first, second int
}

func PalindromePairs(words []string) [][]int {
	ans := [][]int{}
	stack := map[string]combined_words{}

	for x, i := range words {
		for y, j := range words {
			word := fmt.Sprintf("%s%s", i, j)
			stack[word] = combined_words{x, y}
			word = fmt.Sprintf("%s%s", j, i)
			stack[word] = combined_words{y, x}
		}
	}

	//check mirrored strings
	for key, val := range stack {
		if len(key)%2 != 0 {
			continue
		}
		half_len := len(key) / 2
		first_half := key[0:half_len]
		second_half := key[half_len:]

		if first_half == second_half {
			ans = append(ans, []int{val.first, val.second})

		}
	}

	return ans
}
