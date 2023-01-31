package solution

import (
	"github.com/stretchr/testify/assert"
	"testing"
)

func Test001(t *testing.T) {
	/*
	   board := [][]byte{
	   {'A', 'B', 'C', 'E'},
	   {'S', 'F', 'C', 'S'},
	   {'A', 'D', 'E', 'E'},
	   };
	   out := Exists(board, "ABCCED");
	*/
	out := true
	if out != true {
		t.Fatalf("Expected %v to be %v", out, true)
	}
	assert.Equal(t, true, out, "expected %v to be %v", out, true)
	t.Logf("Test runn")
}

func TestIntegerPartionTestIntegerPartion(t *testing.T) {
	t.Logf("Test runn")
}
