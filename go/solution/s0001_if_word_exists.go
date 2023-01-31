package solution

import "log"
type idx struct {
  x int
  y int
}

func Exists(board [][]byte, word string) bool {
  pos := idx {x:0, y:0}
  x, y := len(board), len(board[0]);
  curr := 0
  moveLeft := func (x idx) byte { 
    if (x.x > 1) {
    return board[x.x - 1][x.y]
    } 
    return '0'
  }
  moveRight := func (z idx) byte { 
    if (z.x>x - 1) {
      return board[z.x + 1][z.y]
    } 
    return '0'
  }
  moveUp := func (x idx) byte {
    if (x.y > 1) {
      return board[x.x][x.y - 1]
    } 
    return '0'
  } 
  moveDown := func (z idx) byte {
    if (z.y > y - 1) {
      return board[z.x][z.y + 1]
    } 
    return '0'
  }

  for curr < len(word){
    if (board[pos.x][pos.y] == word[curr]) {
      board[pos.x][pos.y] = '#'
      log.Print(board[pos.x][pos.y])
      curr++
      continue
    }
    switch (word[curr]) {
    case moveLeft(pos):
      pos.x--;
      continue;
    case moveRight(pos):
      pos.x++;
      continue;
    case moveUp(pos):
      pos.y--;
      continue;
    case moveDown(pos):
      pos.y++;
      continue;
  }

    
  }




  return true;
}
