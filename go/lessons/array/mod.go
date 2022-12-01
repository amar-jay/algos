package array

type Array[T any] interface {
	// ReverseArray will reverse the array
	ReverseArray()
	// Fetch a given array
	GetArray() []T
}
