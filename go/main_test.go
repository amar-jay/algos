package main

import (
	"testing"

	"github.com/amar-jay/algos/go/lessons"
	"github.com/amar-jay/algos/go/lessons/array"
	_ "github.com/amar-jay/algos/go/solution"
)

func TestVector(t *testing.T) {
	t.Parallel()

	t.Run("Vector Addition", func(t *testing.T) {
		a := lessons.NewVector2(1, 2)
		b := lessons.NewVector2(1, 2)
		c := a.Add(b)
		if c.X() != 2 && c.Y() != 4 {
			t.Errorf("Expected 2 as x, got %f, Expected 4 as y, got %f", c.X(), c.Y())
		}
	})

	t.Run("Vector Subtraction", func(t *testing.T) {
		a := lessons.NewVector2(1, 2)
		b := lessons.NewVector2(1, 2)
		c := a.Sub(b)
		if c.X() != 0 && c.Y() != 0 {
			t.Errorf("Expected 0 as x, got %f, Expected 0 as y, got %f", c.X(), c.Y())
		}
	})

	t.Run("Vector Multiplication", func(t *testing.T) {
		a := lessons.NewVector2(1, 2)
		b := lessons.NewVector2(1, 2)
		c := a.Mul(b)
		if c.X() != 1 && c.Y() != 4 {
			t.Errorf("Expected 2 as x, got %f, Expected 4 as y, got %f", c.X(), c.Y())
		}
	})

	t.Run("Vector Division", func(t *testing.T) {
		a := lessons.NewVector2(1, 2)
		b := lessons.NewVector2(1, 2)
		c := a.Div(b)
		if c.X() != 2 && c.Y() != 1 {
			t.Errorf("Expected 1 as x, got %f, Expected 1 as y, got %f", c.X(), c.Y())
		}
	})
}

func TestArray(t *testing.T) {
	t.Parallel()

	t.Run("Reverse Array", func(t *testing.T) {
		a := array.NewArray([]int{1, 2, 3, 4, 5})
		b := array.NewArray([]int{5, 4, 3, 2, 1})
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
