#include <stdio.h>

main()
{
    int c;

    while ((c = getchar()) != EOF) putchar(c);
    
    printf("getchar() != EOF: %d\n", getchar() != EOF);
}