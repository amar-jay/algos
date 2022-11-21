#include <stdio.h>
#include <stdlib.h>
#include<unistd.h>
//#include<windows.h> // if operating system is windows
#include <time.h>
#include <assert.h>
#include <stdbool.h>

int test_017();
bool contains(int* arr ,int len,int num);
int generate_random_value(int begin, int end);
int soru_017() {
  return test_017();
}

bool contains(int* arr ,int len,int num) {
  int size = sizeof(int);

  for (int i=0; i<len; i++) {
    if (*(arr+i) == num) {
      return true;
    }
}

  return false;
}
int generate_random_value(int begin, int end) {
  srand(time(NULL));

  return begin + (rand() % (end - begin + 1));

}
int test_017() {
  int num, start, end, tries = 0;
  puts("what is the range? start end");
  scanf("%d %d", &start, &end);
  puts("wait untl what value?");
  scanf("%d", &num);
  int generated = generate_random_value(start,end);

  while (generated != num) {
    ++tries;
    printf("generateed: %d\n", generated);
    generated = generate_random_value(start,end);
    sleep(1);
  }
    printf("finally generated: %d\n", generated);
  return 0;
}
