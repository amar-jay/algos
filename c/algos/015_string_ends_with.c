/*
 *Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).
 */

#include <stdbool.h>

bool solution(const char *string, const char *ending)
{
    return true;
}

struct String { 
    char* ptr; 
    int len; 
  }; 

String new_str(char* ptr, int len) {
  struct String str;
  str.ptr = ptr;
  str.len = len;
  return str;
}

int soru_015() {
  if (solution("", ""))
    printf("got that right");
  else
    printf("That is ❌");



}
