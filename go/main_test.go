package main
import (
  "testing"
  )


func checkAnagram(str string, list []string, ans []string) bool {
  if ans == nil || list == nil {
    return false
  }

  anagrams := Anagrams(str, list)
  for _,x := range anagrams {
    for _, a := range ans {
      if x != a {
        return false
      }
    } 
  }
    return true
}

func TestingAnagram(t *testing.T){
     checkAnagram("abba", []string{"aabb", "abcd", "bbaa", "dada"}, []string{"aabb", "bbaa"})
     checkAnagram("laser", []string{"lazing", "lazy",  "lacer"}, nil)

}

