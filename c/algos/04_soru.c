#include<stdio.h>

// Soru:
// Programcıya 10 sayı girdiren ve en büyüğünü bulup
// ekrana yazdıran program.
int dorduncu_soru() {
    int en_buyuk, sayi;

    for (int i = 0; i< 10; i++) {
	puts("Sayilari girin");
       	scanf("%d", &sayi);
	if (en_buyuk < sayi) en_buyuk = sayi;
    }

    printf("En Büyük sayı:\t%d", en_buyuk);
    return 0;
}
