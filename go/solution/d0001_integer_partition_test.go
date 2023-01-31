package solution

import (
	"testing"

	"github.com/amar-jay/algos/go/utils"
)

func Test(t *testing.T, n int, res string) bool {
	sol := Part(n)
	ans, err := utils.AssertEqual(sol, res)
	if err != nil {
		t.Fatalf("Actual: (%s) Expected (%s)", sol, res)
		return false
	}
	return ans
}

/*
func TestIntegerPartionTestIntegerPartion(t *testing.T) {
      test(t, 1, "Range: 0 Average: 1.00 Median: 1.00")
      test(t, 2, "Range: 1 Average: 1.50 Median: 1.50")
      test(t, 3, "Range: 2 Average: 2.00 Median: 2.00")
      test(t, 4, "Range: 3 Average: 2.50 Median: 2.50")
      test(t, 5, "Range: 5 Average: 3.50 Median: 3.50")
      test(t, 6, "Range: 8 Average: 4.75 Median: 4.50")

}*/
