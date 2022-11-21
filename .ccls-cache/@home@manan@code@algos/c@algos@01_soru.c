#include <stdio.h>


// Soru:
// İki tamsayı alan ve birinci tamsayının ikinci 
// tamsayının tam katı olup olmadığını hesaplayan 
// ve sonucu ekranda gösteren program.
int birinci_soru() {
    int sayi[5];

    // Sayilari girin
    for (int i = 0; i< 5; i++)
	scanf("%d", &sayi[i]);

    // En buyuk ve en kucuk bulun
    int buyuk = sayi[0];
    int kucuk = sayi[0];
    for (int i = 0; i< 5; i++) {
	if (sayi[i] > buyuk)
	    buyuk = sayi[i];
	if (sayi[i] < kucuk)
	    kucuk = sayi[i];
    }

    printf("En buyuk: %d\nEn kuçuk: %d\n", buyuk, kucuk);

   return 0; 
}
