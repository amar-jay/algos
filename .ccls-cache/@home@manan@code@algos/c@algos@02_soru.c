#include <stdio.h>

// Soru:
// İki tamsayı alan ve birinci tamsayının
// ikinci tamsayının tam katı olup olmadığını 
// hesaplayan ve sonucu ekranda gösteren program.
int ikinci_soru() {
    int birinci, ikinci;

    printf("Birinci sayi\t");
    scanf("%d", &birinci);
    printf("Ikinci sayi\t");
    scanf("%d", &ikinci);

//     olup olmadığını kontrol edin.
    if (birinci % ikinci) {
	printf("Tam kat sayi değil\n");
	return 1;
    }

    printf("Tam kat sayidir\n");
    return 0;
}
