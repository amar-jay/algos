#include<stdio.h>

int aliter(int arr[] , int start, int end, int val){
  int len = end - start;
  int mid = len / 2;

  if (arr[mid] == val){
    printf("%d with index %d\n", val, start+mid);
    return 0;
  }
  if (arr[mid] < val){
    return aliter(arr+ mid, start+mid, end, val);
  }

  if (arr[mid] > val){
    return aliter(arr, start, start+mid, val);
  }

  if (len <= 1) {
    printf("Not found");
    return 1;
  }
  return 1;
}

int binary_search(int arr[] , int start, int end, int val){
  while (start <= end) {
    int mid = (start + end) /2;

    if (arr[mid] == val) {
      printf("%d with index %d\n", val, mid);
      return start+mid;
    }

    if (arr[mid] > val) {
      end = mid - 1;
      continue;
    }

      start = mid + 1;
  }
  return 0;
}
