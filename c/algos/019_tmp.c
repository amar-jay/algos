#include <stdio.h>
int soru_019() {
    int sayi;
    int kat = 1;
    int toplam = 0;
    printf("2lik sayiyi giriniz:\t");
    scanf("%d", &sayi);
    while (sayi > 0) {
      int bd = sayi % 10;
      if (bd != 0 && bd != 1) {
	printf("yanlizca 0 ve 1 giriniz! \n");
	return 1;
      }
      sayi /= 10;
      toplam += kat * bd;
      kat *= 2;
    }
    printf("10luk karsiligi:\t%d\n", toplam);
  return 0;
}

