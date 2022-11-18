#include<stdio.h> 
#include<math.h> 

// Soru:
// Sadece 0 ve 1’lerden oluşan bir tamsayı (ikilik sistem) girişi
// yaptırın ve bu sayıyı 10’luk sistemde yazdıran program.
// (İpucu: mod ve bölme operatörlerini kullanarak sayının
// basamaklarını teker teker sağdan sola doğru alabilirsiniz.
// 10’luk sistemde en sağdaki sayının pozisyon değeri 1 ve sonrakilerin 
// 10, 100, 1000 olacak şekilde 10’un kuvvetlerinde arttığı gibi,
// ikilik sistemde de 1 ile başlayıp 2 nin kuvvetleri şeklinde,
// 2, 4, 8 gibi artmaktadır. Örneğin 10’luk sistemdeki 234 sayısı
// 4 * 1 + 3 * 10 + 2 * 100 şeklinde gösterilir ve 1101 ikilik 
// sistem sayısının 10’luk sistemdeki karşılığı
// 1 *1 + 0 * 2 + 1 * 4 + 1 * 8 ya da 1 + 0 + 4 + 8 yada 13’ tür.)
int sekizinci_soru() {
    int num,kat=1;
    int onluk=0;

    int sayi;
    puts("sayi girin");
    scanf("%d", &sayi);

    // example: - 101
    // 1 -> 1 * 1 = 1 
    // 0 -> 0 * 2 = 0
    // 1 -> 1 * 4 = 4
    // ans = 1 + 0 + 4;
    // iterate over the number
    while (sayi > 0) {
	int num = sayi%10;
	onluk += num * kat;

	sayi = (int) sayi/10;
	kat *= 2;
    }
    
	printf("Onluk Sayi: %d\n", onluk);


}
