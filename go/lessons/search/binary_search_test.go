package search

import "testing"

func TestBinarySearch(t *testing.T) {
	//	t.Parallel()
	arr := []int{1, 3, 5, 6, 7}
	t.Run("Find Figure", func(t *testing.T) {
		ans := BinarySearch(arr, 1)
		if ans != 1 {
			t.Errorf("Expected %d to be %d", ans, 1)
		}
	})

}
