package array;

/** https://www.geeksforgeeks.org/write-a-program-to-Reverse-an-array-or-string/
 * This function will accept an array and
 * Reverse its elements and returns the inverted array
 * @param {Array} arr array with elements of any data type
 * @returns {Array} array with inverted elements
 */
type array[T any] struct {
  arr []T
  }

func NewArray[T any](x []T) Array[T] {
  return &array[T]{arr: x}
}

func (a *array[T])GetArray() []T {
  return a.arr
}

func (a *array[T])ReverseArray() {
  for i := range a.arr[:len(a.arr)/2] {
    tmp := a.arr[i];
    a.arr[i] = a.arr[len(a.arr)-1-i];
    a.arr[len(a.arr)-1-i] = tmp;
  }
}
