import PaginationHelper from '../d002_pagination_helper';
describe("Pagination helper", () => {
  let func = PaginationHelper(['a','b','c','d','e','f'], 4);

  test("item count test", () => expect(func.itemCount()).toBe(5));
  test("page count test", () => expect(func.pageCount()).toBe(1));
  test("page item count test", () => expect(func.pageItemCount(-1)).toBe(4));
  test("page item count test", () => expect(func.pageItemCount(0)).toBe(2));
  test("page item count test", () => expect(func.pageItemCount(1)).toBe(-1));
  test("index 4 page", () => expect(func.pageIndex(5)).toBe(1));
  test("index 5 page", () => expect(func.pageIndex(6)).toBe(1));
  test("index 6 page", () => expect(func.pageIndex(7)).toBe(-1));
  test("page 0 index", () => expect(func.pageIndex(2)).toBe(0));
  test("page invalid index", () => expect(func.pageIndex(199)).toBe(-1));
  test("page invalid index", () => expect(func.pageIndex(-10)).toBe(-1));




})
