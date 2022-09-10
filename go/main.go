package main


func check_keys(str_map , word_map []rune) bool {
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
  ans := []string {}
  for _, str := range words {
    str_map := []rune(str)
    if check_keys(str_map, word_map) {
      ans = append(ans, str)
    } 
   
  }
	  // your code
  return ans
}

func main(){
  
    x:= Anagrams("abba", []string{"aabb", "abcd", "bbaa", "dada"})
    y := []string{"aabb", "bbaa"}
  println("main")
  for _,a := range x {
  println("main")
    for _,b := range y {
  println("main %")
      print("hello %s %s", a,b)
  }
  }
}
