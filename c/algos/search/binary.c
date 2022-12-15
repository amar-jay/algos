#include<stdio.h>

int binary_search(int arr[] , int start, int end, int val){
  int len = end - start;
  int mid = len / 2;

  if (arr[mid] == val) {
    printf("index of [%d] is %d\n", val, start+mid);
    return mid;
  }
  if (len < 1) {
    printf("Not found");
    return 1;
  }
  if (arr[mid] > val) {
    binary_search(arr, 0, mid, val);
    return 0;
  }

  binary_search((arr+mid), start+mid, end, val);
  return 0;
}

