package search

// /link
// /https://github.com/TheAlgorithms/Algorithms-Explanation/blob/master/en/Search%20Algorithms/Binary%20Search.md
/*
func binary_search[T comparable](arr []T) T {
	//arr_len := len(arr);
	//first_half := binary_search(arr[0:arr_len/2]);
	//second_half := binary_search(arr[arr_len/2:]);

	if len(arr) == 1 {
		return arr[0]
	}

	return []T{}[0]
}
*/

func aliter[T comparable](arr []T, target T) T {
	len := len(arr)
	// assertion that the length odd number
	if len%2 == 0 {
		panic("odd number len only")
	}
	mid_pt := arr[len/2]
	for mid_pt != target {
		len /= 2
		arr = arr[0:len]
		mid_pt = arr[len/2]
	}
	return arr[0]
}

func BinarySearch[T comparable](arr []T, tar T) T {

	return aliter(arr, tar)
}
