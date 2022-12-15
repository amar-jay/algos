#include<stdio.h>

int linear_search(int arr[] , int len, int val){

  for (int i=0; i<len; i++) {
    if (arr[i] == val){
      printf("index of [%d] is %d\n", val, i);
      return i;
    }
  }
  return 0;
}
