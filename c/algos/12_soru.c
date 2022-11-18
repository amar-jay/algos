#include<stdio.h>
#include<math.h>

// Soru:
// (Pisagor teoremi) Bir dik üçgen, tamamı tamsayılar olan
// üç kenara sahip olabilir. Bu üç kenar, iki kenarın kareleri 
// toplamı hipotenüsün karesine eşittir,bağıntısını sağlamalıdır.
// Kenar1, Kenar2 ya da hipotenüsü 500’den 
// büyük olmayan tüm dik üçgenleri bulan program.
int soru_12() {
    double sinir = 500;

    // Loop sayilar 1dan 500ye kadar
    for (int i=1; i<sinir; i++) {
	for (int j=i; j<sinir; j++) {
	    int i_kare = i * i;
	    int j_kare =  j * j;
	    if (i_kare + j_kare > sinir) break;

	    printf("kenar1:%d\tkenar2:%d\t hipoten:%d\n", i, j, i_kare + j_kare);
	}
    }



    return 0;
}
