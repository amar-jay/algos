package main

import (
  "fmt"
  _ "github.com/amar-jay/algos/go/lessons"
  _ "github.com/amar-jay/algos/go/lessons/search"
    "github.com/amar-jay/algos/go/solution"
  _ "github.com/amar-jay/algos/go/utils"
)

func check_keys(str_map, word_map []rune) bool {
    for key := range str_map {
	    for idx := range word_map {
		    if idx != key {
			    return false
		    }
	    }
    }
    return true
}

func Anagrams(word string, words []string) []string {
    word_map := []rune(word)
    ans := []string{}
    for _, str := range words {
	    str_map := []rune(str)
	    if check_keys(str_map, word_map) {
		    ans = append(ans, str)
	    }

    }
    // your code
    return ans
}

func main() {
    solution.Run()
x := Anagrams("boy", []string{"boy", "girl"})
fmt.Printf("%s", x[0]);

}
