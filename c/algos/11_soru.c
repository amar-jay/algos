#include<stdio.h>

int soru_11() {
    int sayma=0, toplam=0;

    // Forever loop
    for (;;) {
	int n;
	scanf("%d", &n);

	if (n == 9999) {
	    break;
	}
    
	toplam += n;
	sayma++;
    }

    printf("Ortalam sayi: %.4f\n", (double) toplam/sayma);
    return 0;
}
