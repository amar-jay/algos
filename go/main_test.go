package main

import (
	"github.com/amar-jay/algos/go/solution"
	"testing"
)

func TestBanana(t *testing.T) {
  //a := solution.NewPalindromeTest(t)
}

func testMango(t *testing.T) {

  a := solution.PalindromePairs(t)
  t.Fatalf(`Error in palindrome`)
}

func testOrange(t *testing.T) {
  //a := solution.NewPalindromeTest(t)
  t.Fatalf(`Error in palindrome`)
}

func TestSolutions(t *testing.T) {
  println("testing solution")
  solution.PalindromeTests(t);
}
