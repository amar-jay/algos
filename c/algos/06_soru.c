#include<stdio.h>


// Soru:
// ir karenin kenarını kullanıcıdan alan ve o kareyi yıldız 
// karakterlerinden oluşacak şekilde çizen bir program.
int altinci_soru() {
    int sayi;
    puts("sayi girin");
    scanf("%d", &sayi);

    for (int i = 1; i<= sayi*sayi; i++) {
	printf("* ");
	if (i%sayi == 0) printf("\n");
    }


}
