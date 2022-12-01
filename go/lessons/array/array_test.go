package array

import "testing"

func TestArray(t *testing.T) {
	//	t.Parallel()

	t.Run("Reverse Array", func(t *testing.T) {
		a := NewArray([]int{1, 2, 3, 4, 5})
		b := NewArray([]int{5, 4, 3, 2, 1})
		len := len(a.GetArray())
		a.ReverseArray()
		a_arr := a.GetArray()
		b_arr := b.GetArray()

		for i := 0; i < len; i++ {
			if a_arr[i] != b_arr[i] {
				t.Errorf("Expected %v, got %v", a, b)
			}
		}
	})

}
