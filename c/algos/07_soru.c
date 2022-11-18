#include <stdio.h>

// Soru:
// Tersten ve düzden okunduğunda aynı okunan kelimelere palindrom
// denir. Örneğin, 12321, 55555, 45554, 11611 beş basamaklı tam
// sayıları birer palindrom’dur. Kullanıcının girdiği beş basamaklı 
// bir sayının palindrom olup olmadığına karar verip ekrana
// yazdıran bir program.
int yedinci_soru() {

    char sayi[5];
    // Get number
    puts("sayi girin(beş basmaklı olmalı)");
    scanf("%s", sayi);

    for (int i = 0; i<3; i++) {
	if (sayi[i] != sayi[4-i]) {
	    printf("[ %s ] Palidrom değil", sayi);
	    return 1;
	}
    }

    printf("[ %s ] Palidrom sayidir\n", sayi);

    return 0;
}
