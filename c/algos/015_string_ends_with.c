/*
 *Complete the solution so that it returns true if the first argument(string) passed in ends with the 2nd argument (also a string).
 */

struct String { 
    char* ptr; 
    int len; 
  }; 

struct String new_str(char* ptr, int len) {
  struct String str;
  str.ptr = ptr;
  str.len = len;
  return str;
}

int str_cmp(struct String str1, struct String str2) {
	int i;
	for (i = 0; i < str1.len; i++) {
		if (str1.ptr[i] != str2.ptr[i]) {
			return 0;
		}
	}
	return 1;
}

int test() {
  struct String str1 = new_str("abc", 3);
  struct String str2 = new_str("abd", 3);
  if (str_cmp(str1, str2)) {
    printf("Length is %d\n", str1.len);
    printf("String is ");
    for (int i = 0; i < str1.len; i++) {
	printf("%c", str1.ptr[i]);
    }
    puts("");
    return 0;
}
    printf("That is ❌\n");
  return 1;
}
int soru_015() {
	return test();
}
