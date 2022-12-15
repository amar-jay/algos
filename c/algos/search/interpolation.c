#include<stdio.h>
int interpolation_search(int arr[], int start, int end, int val) {
  int mid;

  while (start <= end) {
    int dff = end - start;
    mid = start + (dff) * ((val-arr[start])/(arr[end]-arr[start]));
    if (arr[mid] == val){
      printf("%d with index %d\n", val, mid);
      return mid;
    }

    if (arr[mid] > val){
      end = mid-1;
    }
    start = mid+1;
  }

  return 1;
}
