# Sorting

## Classification of sorting algos

Sorting Algos or all Algos in general are classified based by their;
- Big - 0 - number of loops
- Amount of extra space required

## Sorting algos

- [x] Selection Sort
- [x] Bubble Sort
- [x] Insertion Sort
- [ ] Quick Sort
- [ ] Bogo Sort
- [ ] Bucket Sort
- [ ] Bubble Sort
- [ ] Cocktail Shaker
- [ ] Comb Sort 
- [ ] Counting 

---
## [Bubble Sort](./bubble_sort.rs) 

A bubble sort algorithm is one of the simplest type of algorithm.
Sorts by swaping elements. Efficient for small data.[Read more..](https://www.geeksforgeeks.org/bubble-sort/)
Time complexity is `O(n^2)`, where `n` is the number of elements.
Space complexity is `O(1)` as it sorts elements in-place

#### Steps

1. Check if array is empty
2. Store the size of arr in `N`
3. Loop as long as the array isn't sorted
  - Swap each element element with the preceding one until if greater or maintain it if not 
  - reduce `N` by 1 

---
## [Bucket Sort](./bucket_sort.rs)

Bucket sort is a comparison sort algorithm that operates on 
elements by dividing them into different buckets and then
sorting these buckets individually. Bucket sort is mainly useful
when the input is  uniformly distributed over a range

### Steps

- Create **n** empty buckets
- Put array elements in different buckets
- Sort individual buckets
- Concatenate all buckets into arr[]

---
## [Counting Sort](./counting_sort.rs)

The counting sort algorithm works by first creating a list
of the counts or occurrences of each unique value in the list.
One important thing to remember is that counting sort can only 
be used when you know the range of possible values in the input 
beforehand.

### Steps

**TODO**

---
## [Selection Sort](./selection_sort.rs)

### Steps

**TODO**

---
## [Insertion Sort](./insertion_sort.rs)

Also another simple sorting algos. Efficient for small data
Sorts a mutable slice using in-place insertion sort algorithm. [Read more..](https://www.geeksforgeeks.org/insertion-sort/)
Time complexity is `O(n^2)`, where `n` is the number of elements.
Space complexity is `O(1)` as it sorts elements in-place.

#### Steps

`[B, A, D, C]`
1. first every element,`X` in array. 
  - If `B` > `A`.`[A, B, D, C]`
2. `B` not > `D`. Stays the same. And same for `C`. Repeat Step 1. and 2. for B, then C and D until sorted

---
## [Quick Sort](./quick_sort.rs)

Time complexity is `O(nlog(n))` with worst case `O(n^2)`, where `n` is the number of elements.

#### Steps

**TODO**


