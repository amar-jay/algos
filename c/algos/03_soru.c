#include<stdio.h>

// Soru:
// Bir ilaç şirketi, satış elemanlarına ücretlerini prim şeklinde
// ödemektedir. Bir satış elemanı haftalık 200$ ve haftalık brüt
// satışından %9 prim almaktadır. Örneğin, 5000$ tutarında bir
// haftalık satış yapan satış elemanı 200$ ve 5000$’in %9’unu 
// kazanmaktadır, yani 650$. Son haftadaki satış elemanlarının 
// satışlarını kullanıcıya girdiren ve bu satış elemanlarının ne
// kadar kazandıklarını hesaplayıp ekrana yazdıran bir program
// yazınız. Her seferinde bir satış elemanı için işlemleri yapınız.

int ucuncu_soru() {
    double haftalik = 200, satis;

    while (1== 1) {
	printf("Dolar cinsiden satış tutarını giriniz(Çikiş için -1): ");
	scanf("%lf", &satis);

	if (satis == -1) break;
	printf("Maaş: %.2f$\n", 0.09*satis + 200);
    }


    return 0;
}
