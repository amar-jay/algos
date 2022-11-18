#include<stdio.h> 

//Soru:
//Bir şirket, müdürlerine (sabit haftalık ücret alırlar), vardiyalı
//işçilerine (40 saate kadar sabit saatlik ücret ve sonrası için 
//saatlik ücretin 1.5 katı alırlar), komisyon işçilerine 
//(sabit 250$ ve haftalık brüt satışların %5.7 sini alırlar) yada
//parça işçilerine (ürettiği her malzeme başına sabit ücret alırlar 
//-- her parça işçisi tek bir malzeme üzerinde çalışır--)
//maaş vermektedir. Her çalışan tipinin haftalık maaşını hesaplayan 
//bir program yazınız. Kaç çalışan olduğunu bilmiyorsunuz. Her tipte
//çalışanın kendi maaş kodu vardır. müdürler 1, vardiyalı işçiler 2,
//komisyon işçileri 3 ve parça işçileri 4. Bu maaş kodlarına göre
//çalışanların maaşları hesabı için, switch yapısı kullanın. 
//switch içerisinde kullanıcının gerekli değerleri girmesini sağlayın.
// // NOT DONE
// ANlamiyorum saru
int soru_13() {
    int hafatalik, tip;
    unsigned int count;
    float fiyat, satis;
    int ucret, saat, sabit;
    puts("Müdürler\t\t1\nVardiyalı işçiler\t2 \nKomisyon işçileri\t3 \nParça işçileri\t\t4");
    puts("Birini seçin");
    scanf("%d", &tip);

    switch (tip) {
	case 1:
	    puts("Müdür");
	    puts("haftalık ücret kaç alırsın");
	    scanf("%d", &ucret);
	    printf("Haftalik ücretin %d\n", ucret);
	    break;
	case 2:
	    puts("vardiyal işçi ");
	    puts("kaç saat çalıştın bu hafta");
	    scanf("%d", &saat);
	    puts("bir saatta sabit saatlik ücreti girin");
	    scanf("%d", &sabit);
	    if (saat < 40) {
		printf("Haftalik ucreti %.2d$\n", sabit*saat);
		return 0;
	    }
	    printf("Haftalik ücretin %.2f\n", (sabit*40) + ((saat - 40)* 1.5 * sabit));
	    break;
	case 3:
	    puts("Komisyon işçileri");
	    puts("haftalık ücretin girin");
	    scanf("%f", &satis);
	    printf("Haftalik ücretin: %.2f\n", 250 + 0.057*satis);
	    break;
	case 4:
	    /*
	     parça işçilerine (ürettiği her malzeme başına sabit 
	     ücret alırlar -- her parça işçisi tek bir 
	     malzeme üzerinde çalışır--) maaş vermektedir.
	     */
	    puts("Parça işçiler");
	    puts("ürettiği bir malzeme için fiyati");
	    scanf("%f", &fiyat);
	    puts("Kaç tane üretilmiş");
	    scanf("%d", &count);

	    printf("Haftalik ücretin: %.2f\n", fiyat * count);
	    break;
	default:
	    puts("Bu  işci tipi yoktur");
	    return 1;
    }



    return 0;
}
