package utils

import "testing"

func TestTestFunctions(t *testing.T) {
	t.Run("Testing Assert", func(t *testing.T) {
		// equal
		val, err := AssertEqual(4, 4)
		if !val {
			t.Errorf("Assert function failed for ints\nError:%s", err)
		}

		// equal
		val, err = AssertEqual(4.55, 4.550)
		if !val {
			t.Errorf("Assert function failed for floats\nError:%s", err)
		}

		// not equal
		val, err = AssertEqual("mango", "banana")
		if val {
			t.Errorf("Assert function failed for strings\nError:%s", err)
		}

	})

}
