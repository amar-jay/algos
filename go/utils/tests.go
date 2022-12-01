package utils

import (
	"errors"
	"fmt"
)

func AssertEqual[T comparable](actual T, expected T) (bool, error) {
	if actual != expected {
		return false, fmt.Errorf("actual: (%v) is not equal to expected (%v)", actual, expected)
	}
	return true, nil
}

func AssertEqualArray[T comparable, S []T](actual S, expected S) (bool, error) {
	if len(actual) == 0 {
		return false, errors.New("actual is empty")
	}

	if len(expected) == 0 {
		return false, errors.New("expected is empty")
	}

	for _, i := range actual {
		for _, j := range expected {
			if i != j {
				return false, fmt.Errorf("expected string(%v) is not same as"+
					" actual string (%v)", expected[0], actual[0])
			}

		}
	}
	return true, nil
}

func AssertNotEqual[T comparable](actual T, expected T) (bool, error) {
	if actual == expected {
		return false, fmt.Errorf("actual: (%v) must not be equal to expected (%v)", actual, expected)
	}

	return true, nil
}

func AssertNotEqualArray[T comparable, S []T](actual S, expected S) (bool, error) {
	if len(actual) == 0 {
		return false, errors.New("actual is empty")
	}

	if len(expected) == 0 {
		return false, errors.New("axpected is empty")
	}

	for _, i := range actual {
		for _, j := range expected {
			if i != j {
				return true, nil
			}
		}
	}
	return false, fmt.Errorf("expected (%v) is not same as"+
		" actual string (%v)", expected[0], actual[0])
}

/*
func NewTest(t *testing.T) Test {
  return Test{T:t}
}
*/
