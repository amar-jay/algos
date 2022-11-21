#include <stdio.h>
#include<unistd.h>
#include <time.h>
#include <stdlib.h>

enum options {
  Rock,
  Paper,
  Scissors
};
struct choices {
  enum options player1;
  enum options player2;
};
enum results {
  Win,
  Draw,
  Loss
};

/// choose random  rock, paper, scissors
struct choices random_choice();
enum results who_is_the_winner(enum options p1, enum options p2);
int soru_018() {
  char choices[3][10] = {"rock","paper", "scissors" };
  srand(time(NULL));
  struct choices choice = random_choice();
  int res = who_is_the_winner(choice.player1, choice.player2);
  printf("player1: %s\tplayer2: %s\n", choices[choice.player1], choices[choice.player2]);

  if (res == Win) {
    printf("Player 1 has won the tournament");
  } else if (res == Loss)
    printf("Player 2 has won the tournament");
  else 
    printf("Player 1 and player 2 draw");

    puts("");
  return 0;
}


/// 0 -> Draw
/// 1 -> player1
/// 2 -> player2
enum results who_is_the_winner(enum options p1, enum options p2) {

  enum options o;
  switch (p1) {
    case Paper:
      if (p2 == Rock) return Win;
      else if (p2 == Paper) return Draw; 
	else return Loss;  
    break;
    case Rock:
      if (p2 == Paper) return Win;
      else if (p2 == Rock) return Draw; 
	else return Loss;  
    break;
    case Scissors:
      if (p2 == Paper) return Win;
      else if (p2 == Scissors) return Draw; 
	else return Loss;  
    default:
    return Loss;
  }
}

struct choices random_choice() {
  enum options opts[3] = {Rock, Paper, Scissors};
  struct choices c;
  c.player1 = opts[rand() % 3];
  sleep(1);
  c.player2 = opts[rand() % 3];

      return c;
}
