#include <stdio.h>
#include <stdlib.h>
#include <time.h>
#include <assert.h>
#include <stdbool.h>

int test_016();
bool contains(int* arr ,int len,int num);
int generate_random_number(int begin, int end);
int soru_016() {
  return test_016();
}

bool contains(int* arr ,int len,int num) {
  int size = sizeof(int);

  for (int i=0; i<len; i++) {
    printf("n : %d\n", *(arr+i));
    if (*(arr+i) == num) {
      return true;
    }
}

  return false;
}
int generate_random_number(int begin, int end) {
  srand(time(NULL));

  return begin + (rand() % (end - begin));

}
int test_016() {
  int arr[7] = {1, 2, 3, 4, 5, 6, 7};
  int generated = generate_random_number(0,7);
  printf("generateed: %d\n", generated);
  printf("size of arr is : %ld\n", sizeof(arr)/sizeof(int)); 
  assert(contains(arr, sizeof(arr)/sizeof(int), generated));

  return 0;
}
