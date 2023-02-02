#include<stdio.h>


// Soru:
// 1, sayi > 0
// 2, factorial hesapla - Done
// 3, e hasaplama
// 4, e^x hesaplama
int onuncu_soru() {

    int factorial = 1,sayi;
    puts("factorial bulmak için sayi girin");
    scanf("%d", &sayi);
    
    if (sayi < 0) {
	puts("Sayi positif olmalidir");
	return 1;
    }

    for (int i = 1; i<= sayi; i++) {
	factorial *= i;
    }

    printf("Factorial: %d\n", factorial);

}
