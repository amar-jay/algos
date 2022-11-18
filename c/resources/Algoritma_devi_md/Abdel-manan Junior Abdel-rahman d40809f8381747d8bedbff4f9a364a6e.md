# Abdel-manan Junior Abdel-rahman

### Öğrenci No: 170521923

## Algoritma Ödevi

## Soru 1

### ****************Söz kodu****************

- initialize an integer array of size 5
- loop from 0 to 4
    - döngü sayımına göre array’de girdi al ve kaydet
- enbuyuk = arrayin birinci sayi
- enkucuk = arrayin birinci sayi
- diziyi döngüye alın ve bir sayi en büyük olup olmadığını kontrol edin, büyük olanı kaydedin. En küçük bulmak için ayni şey yapın.
- Print en büyük ve en küçük

### Flowgorithma

[xx.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/xx.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/01_soru.c)********

```c
int birinci_soru() {
    int sayi[5];

    // Sayilari girin
    for (int i = 0; i< 5; i++)
	scanf("%d", &sayi[i]);

    // En buyuk ve en kucuk bulun
    int buyuk = sayi[0];
    int kucuk = sayi[0];
    for (int i = 0; i< 5; i++) {
	if (sayi[i] > buyuk)
	    buyuk = sayi[i];
	if (sayi[i] < kucuk)
	    kucuk = sayi[i];
    }

    printf("En buyuk: %d\nEn kuçuk: %d\n", buyuk, kucuk);

   return 0; 
}
```

## Soru 2

### ****************Söz kodu****************

- Fonksiyon Main
    - declare to store “birinci sayı” and “ikinci sayı”
    - Get input and store in birinci sayi
    - Get input and store in ikinci sayi
- if “birinci sayi” mod (”ikinci sayi”)  == 0
    - Print it is a multiple
- else print it is not a multiple

### **Flowgorithma**

[002.jpg](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/002.jpg)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/02_soru.c)********

```c
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
```

## Soru 3

### ****************Söz kodu****************

- Fonksiyon Main
    - declare and assign integer to store “haftalik ücreti” as 200
    - declare integer to store satiş
- Infinite loop (Sonsuz döngü)
    - Girdi al ve satış’a kaydet
    - If satış == -1 then break loop
    - else print 9% * satis + 200

### Flowgorithma

[003.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/003.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/03_soru.c)********

```c
int ucuncu_soru() {
    double haftalik = 200, satis;

    while (1 == 1) {
	printf("Dolar cinsiden satış tutarını giriniz(Çikiş için -1): ");
	scanf("%lf", &satis);

	if (satis == -1) break;
	printf("Maaş: %.2f$\n", 0.09*satis + 200);
    }

    return 0;
}
```

## Soru 4

### ****************Söz kodu****************

- Fonksiyon Main
    - declare an integer to store “en buyuk” and “sayi”
    - declare integer to store satiş
- Loop from 0 to 9
    - Girdi al ve sayi’a kaydet
    - If enbuyuk < sayi then enbuyuk = sayi
- print enbuyuk

### Flowgorithma

[004.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/004.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/04_soru.c)********

```c
int dorduncu_soru() {
    int en_buyuk, sayi;

    for (int i = 0; i< 10; i++) {
	puts("Sayilari girin");
       	scanf("%d", &sayi);
	if (en_buyuk < sayi) en_buyuk = sayi;
    }

    printf("En Büyük sayı:\t%d", en_buyuk);
    return 0;
}
```

## Soru 5

### ****************Söz kodu****************

- Fonksiyon Main
    - print “A    A+2    A+4    A+6”
    - declare integer to store satiş
- Loop from 0 to 9 where loop count is i
    - print “3*i    3*i+2    3*i+4    3*i+6”

### Flowgorithma

[005.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/005.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/05_soru.c)********

```c
int besinci_soru() {

    puts("A\tA+2\tA+4\tA+6");
    for (int i = 1; i<= 5; i++) {
	printf("%d\t%d\t%d\t%d\n", 3*i, 3*i+2, 3*i+4, 3*i+6);
    }
    return 0;
}
```

## Soru 6

### ****************Söz kodu****************

- Fonksiyon Main
    - print "sayi girin"
    - declare integer as sayi
    - get input into sayi
- Loop from 1 to sayi * sayi where loop count is i
    - print “* ”
    - if (i%sayi == 0) print “\n”

### Flowgorithma

[006.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/006.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/06_soru.c)********

```c
int altinci_soru() {
    int sayi;
    puts("sayi girin");
    scanf("%d", &sayi);

  for (int i = 1; i<= sayi*sayi; i++) {
		printf("* ");
		if (i%sayi == 0) printf("\n");
   }
}
```

## Soru 7

### ****************Söz kodu****************

- Fonksiyon Main
    - declare integer sayi, reverse=0  and n
    - print “sayi girin”
    - get input into sayi
    - n = sayi
- for n= sayi; n ≠ 0 , n = n / 10
    - reverse = reverse * 10 + (n % 10)
    - n = n / 10

if sayi == reverse  then print “palidrom sayıdır”

else print “palidrom sayi değildir”

### Flowgorithma

[007.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/007.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/07_soru.c)********

```c

int diger_yontem() {
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
int yedinci_soru() {
    char sayi[5];
    // Get number
    puts("sayi girin(beş basmaklı olmalı)");
    scanf("%s", sayi);

	for (int n = sayi;n != 0; n/=10) {
		reverse = reverse * 10 + (n % 10)
  }

	if (sayi == reverse)
		printf("[ %s ] Palidrom sayidir\n", sayi);
	else
		printf("Palidrom sayi değil")ş

  return 0;
}
```

## Soru 8

### ****************Söz kodu****************

- Fonksiyon Main
    - declare integer num, sayi, kat=1 , onluk = 0;
    - print “sayi girin”
    - get input into sayi
    - n = sayi
- while sayi > 0
    - onluk = onluk + (num * kat)
    - kat *= 2;
- print onluk

### Flowgorithma

[008.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/008.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/08_soru.c)********

```c
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
```

## Soru 9

### ****************Söz kodu****************

- Fonksiyon Main
    - declare integer n, count, sayi, kat=1 , onluk = 0;
    - print “sayi girin”
    - get input into n
- while n > 0:
    - assign num = n % 10
    - if num == 9 then count = count + 1;
    - n = quotient of (n / 10)

### Flowgorithma

[009.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/009.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/09_soru.c)********

```c
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
```

## Soru 10

### ****************Söz kodu****************

- Fonksiyon Main
    - declare integer factorial = 1, sayi;
    - print “sayi girin”
    - get input into sayi
- if sayi < 0:
    - print “sayi positif olamali”
    
    else 
    
    - loop from 1 to sayi where loop count is i
    - factorial *= i;
    
    print factorial
    

### Flowgorithma

[010.pdf](Abdel-manan%20Junior%20Abdel-rahman%20d40809f8381747d8bedbff4f9a364a6e/010.pdf)

### ********[Code](https://github.com/amar-jay/odevler/blob/master/odev_01/10_soru.c)********

```c
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
```