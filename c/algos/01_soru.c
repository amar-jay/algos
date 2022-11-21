#include <stdio.h>


// Soru:
// İki tamsayı alan ve birinci tamsayının ikinci 
// tamsayının tam katı olup olmadığını hesaplayan 
// ve sonucu ekranda gösteren program.
int birinci_soru() {
  int sayilar[5];



  for (int i=0; i<5;i++)
    scanf("%d", &sayilar[i]);

  int MAX=sayilar[0], MIN=sayilar[0];
  for (int i=0; i<5;i++){
    if (sayilar[i] == MAX)
      MAX = sayilar[i];

    if (sayilar[i] == MIN)
      MIN = sayilar[i];
  }

  printf("MAX: %d\nMIN: %d", MAX, MIN);

  return 0;
}


