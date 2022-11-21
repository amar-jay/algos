#include <stdio.h>
#include <assert.h>

int sol1();
// Soru:
// Tersten ve düzden okunduğunda aynı okunan kelimelere palindrom
// denir. Örneğin, 12321, 55555, 45554, 11611 beş basamaklı tam
// sayıları birer palindrom’dur. Kullanıcının girdiği beş basamaklı 
// bir sayının palindrom olup olmadığına karar verip ekrana
// yazdıran bir program.
int soru_07() {
//  assert(sol1() == sol2());
  sol1(); 
  return 0;
}

int sol1() {

  char sayi[] = {};
  // Get number
  puts("sayi girin(beş basmaklı olmalı)");
  scanf("%s", sayi);

  int len = sizeof(sayi) / sizeof(char);
  for (int i = 0; i< 0.5*len + 1; i++) {
    if (sayi[i] < 48 || sayi[i] > 57 ) {
      puts("sayi olmali");
      return 1;
    }
    if (sayi[i] != sayi[len-i]) {
      printf("[ %s ] Palidrom değil\n", sayi);
      return 1; 
    }
    }

    printf("[ %s ] Palidrom sayidir\n", sayi);

    return 0;
}
