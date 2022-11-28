interface PaginationHelper {
  itemCount(): number 
  pageCount(): void
  pageItemCount(): void
  pageIndex(itemIndex: number): void
  collection: Array<string | number>
  itemPerPage: number

}
function PaginationHelper(this:any | PaginationHelper, collection: Array<string | number>, itemPerPage:number) {
  PaginationHelper.prototype.collection = collection;
  PaginationHelper.prototype.itemPerPage = itemPerPage;
  return {
    itemCount() {
      return PaginationHelper.prototype.collection.length
    },
    pageCount() {
      return Math.ceil(PaginationHelper.prototype.collection.length / PaginationHelper.prototype.itemPerPage);
    },
    pageItemCount(pageIndex:number){
      if (PaginationHelper.prototype.pageCount() > pageIndex+1)
	return	PaginationHelper.prototype.itemPerPage;
      else if (PaginationHelper.prototype.pageCount() == pageIndex+1)
	return (PaginationHelper.prototype.itemCount() % PaginationHelper.prototype.itemPerPage);
      else
	return -1;
    },

    pageIndex(itemIndex:number) {
      return (itemIndex>PaginationHelper.prototype.itemCount())? 0-1 :Math.ceil(itemIndex / PaginationHelper.prototype.itemPerPage) - 1;
    }
  };
}

PaginationHelper.prototype.itemCount = () => {
  return PaginationHelper.prototype.collection.length

}

PaginationHelper.prototype.pageCount = () => {
  return Math.ceil(PaginationHelper.prototype.collection.length / PaginationHelper.prototype.itemPerPage);

}

PaginationHelper.prototype.pageItemCount = (pageIndex:number) => {
  return (PaginationHelper.prototype.pageCount() > pageIndex+1)?
    PaginationHelper.prototype.pageCount(): 
  (PaginationHelper.prototype.itemCount() % PaginationHelper.prototype.pageCount()) - pageIndex;
}

PaginationHelper.prototype.pageIndex = (itemIndex:number) => {
 return (PaginationHelper.prototype.pageIndex>PaginationHelper.prototype.pageCount())?-1:Math.floor(itemIndex / PaginationHelper.prototype.itemsPerPage);
}

export default PaginationHelper;
