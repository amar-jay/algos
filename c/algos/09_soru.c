#include<stdio.h>

// Soru:
// Bir tam sayı alan ve bu tam 
// sayının basamaklarının kaç tanesinin 9 olduğunu bulan
// bir program.
int dokuzuncu_soru() {

    long long int sayi;
    int sayma=0;
    puts("sayi girin");
    scanf("%lld", &sayi);

    while (sayi > 0) {
	// şuanki rakam
	int num = sayi % 10;

	if ( num == 9 ) ++sayma;
	sayi = (int) sayi / 10;
    }

    printf("%d tane 9 var\n", sayma);
    return 0;
}
