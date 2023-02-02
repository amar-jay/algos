#include<stdio.h> 

int soru_14()  {

    int sayi;
    puts("sayi girin");
    scanf("%d", &sayi);

    // 1> s > 19
    if (sayi < 1 || sayi > 19) 
	puts("Sayi 1 ile 19 arasında olmali");

    // Triangle of stars
    /*
     *
    * *
   * * *
  * * * *
 * * * * *
* * * * * *
*/

    for (int i = 1; i <= sayi; i++) {
	for (int j = 1; j <= sayi - i; j++)
	    printf(" ");
	for (int j = 1; j <= i; j++)
	    printf("* ");
	printf("\n");
	}


    for (int i = sayi; i >= 1; i--) {
	for (int j = 1; j <= sayi - i; j++)
	    printf(" ");
	for (int j = 1; j <= i; j++)
	    printf("* ");
	printf("\n");
	}


    return 0;
}
