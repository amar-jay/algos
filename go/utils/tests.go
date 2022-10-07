package utils
import "testing"

type Test struct  {T *testing.T }
func (t Test) AssertEqual(actual [][]int, expected [][]int) {
  if expected == nil {
    t.T.Logf("Expected is null")
  }
  if actual == nil {
    t.T.Logf("Actual is null")
  }
  t.T.Errorf("Actual vs expected")
  for i := range actual {
    for j := range expected {
      if i != j {
        t.T.Errorf("Expected String(%d) is not same as"+
             " actual string (%d)", expected[0],actual[0])
      }

    }
  }
}
func (t Test)AssertNotEqual(actual interface{}, expected interface{}) bool {

  return actual == expected 
}

func NewTest(t *testing.T) Test {
  return Test{T:t}
}



