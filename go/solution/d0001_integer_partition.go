package solution

import "fmt"

/**
From wikipedia https://en.wikipedia.org/wiki/Partition_(number_theory)

In number theory and combinatorics, a partition of a positive integer n, also called an integer partition, is a way of writing n as a sum of positive integers. Two sums that differ only in the order of their summands are considered the same partition.

For example, 4 can be partitioned in five distinct ways:

4, 3 + 1, 2 + 2, 2 + 1 + 1, 1 + 1 + 1 + 1.

We can write:

enum(4) -> [[4],[3,1],[2,2],[2,1,1],[1,1,1,1]] and

enum(5) -> [[5],[4,1],[3,2],[3,1,1],[2,2,1],[2,1,1,1],[1,1,1,1,1]].

The number of parts in a partition grows very fast. For n = 50 number of parts is 204226, for 80 it is 15,796,476 It would be too long to tests answers with arrays of such size. So our task is the following:

1 - n being given (n integer, 1 <= n <= 50) calculate enum(n) ie the partition of n. We will obtain something like that:
enum(n) -> [[n],[n-1,1],[n-2,2],...,[1,1,...,1]] (order of array and sub-arrays doesn't matter). This part is not tested.

2 - For each sub-array of enum(n) calculate its product. If n = 5 we'll obtain after removing duplicates and sorting:

prod(5) -> [1,2,3,4,5,6]

prod(8) -> [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 16, 18]

If n = 40 prod(n) has a length of 2699 hence the tests will not verify such arrays. Instead our task number 3 is:

3 - return the range, the average and the median of prod(n) in the following form (example for n = 5):

"Range: 5 Average: 3.50 Median: 3.50"

Range is an integer, Average and Median are float numbers rounded to two decimal places (".2f" in some languages).

Notes:
Range : difference between the highest and lowest values.

Mean or Average : To calculate mean, add together all of the numbers in a set and then divide the sum by the total count of numbers.

Median : The median is the number separating the higher half of a data sample from the lower half. (https://en.wikipedia.org/wiki/Median)

Hints:
Try to optimize your program to avoid timing out.

Memoization can be helpful but it is not mandatory for being successful.
*/

var (
	see = [][]int{}
)
var (
	sol = []int{}
)

func x(n int) {
	if n == 1 {
		see = append(see, []int{1})
		sol = append(sol, 1)
		return
	}

	if !contains(sol, n) && n > 0 {
		sol = append(sol, 1)
		sol = append(sol, n)
		see = append(see, []int{0, n})
	}

	for i := 1; i <= n; i++ {
		for j := i; j <= n; j++ {
			if i+j == n {
				if !contains(sol, i*j) && i*j > 0 {
					sol = append(sol, i*j)
					see = append(see, []int{i, j})
				}
			}
		}
		if !contains(sol, i*(n-1)) && i*(n-i) > 0 {
			sol = append(sol, i*(n-i))
			see = append(see, []int{i, n - i})
			x(n - i)
		}
	}
}

func contains(arr []int, val int) bool {
	for _, v := range arr {
		if v == val {
			return true
		}
	}
	return false
}

func Part(n int) string {
	x(n)
	ans := sol
	fmt.Printf("n : %d", n)
	fmt.Println(ans)
	fmt.Println(see)
	avg := 0.0
	for _, v := range ans {
		avg += float64(v)
	}
	avg = avg / float64(len(ans))

	return fmt.Sprintf("Range: %d Average: %.2f Median: %.2f", ans[len(ans)-1]-ans[0], avg, avg)
}
