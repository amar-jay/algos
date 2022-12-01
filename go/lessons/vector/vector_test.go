package vector

import "testing"

func TestVector(t *testing.T) {
	//	t.Parallel()

	t.Run("Vector Addition", func(t *testing.T) {
		a := NewVector2(1, 2)
		b := NewVector2(1, 2)
		c := a.Add(b)
		if c.X() != 2 && c.Y() != 4 {
			t.Errorf("Expected 2 as x, got %f, Expected 4 as y, got %f", c.X(), c.Y())
		}
	})

	t.Run("Vector Subtraction", func(t *testing.T) {
		a := NewVector2(1, 2)
		b := NewVector2(1, 2)
		c := a.Sub(b)
		if c.X() != 0 && c.Y() != 0 {
			t.Errorf("Expected 0 as x, got %f, Expected 0 as y, got %f", c.X(), c.Y())
		}
	})

	t.Run("Vector Multiplication", func(t *testing.T) {
		a := NewVector2(1, 2)
		b := NewVector2(1, 2)
		c := a.Mul(b)
		if c.X() != 1 && c.Y() != 4 {
			t.Errorf("Expected 2 as x, got %f, Expected 4 as y, got %f", c.X(), c.Y())
		}
	})

	t.Run("Vector Division", func(t *testing.T) {
		a := NewVector2(1, 2)
		b := NewVector2(1, 2)
		c := a.Div(b)
		if c.X() != 2 && c.Y() != 1 {
			t.Errorf("Expected 1 as x, got %f, Expected 1 as y, got %f", c.X(), c.Y())
		}
	})
}
