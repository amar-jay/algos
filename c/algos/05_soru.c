#include <stdio.h>

// Soru: 
// Aşağıdaki değerler tablosunu döngü kullanarak ekrana yazdıran
// program.
int besinci_soru() {

    puts("A\tA+2\tA+4\tA+6");
    for (int i = 1; i<= 5; i++) {
	printf("%d\t%d\t%d\t%d\n", 3*i, 3*i+2, 3*i+4, 3*i+6);
    }
    return 0;
}
