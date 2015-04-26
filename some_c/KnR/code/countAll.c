#include <stdio.h>

main()
{
    int c, nl, t, b;
    
    nl = t = b = 0;
    while ((c = getchar()) != EOF)
        if (c == '\n') ++nl;
        if (c == '\t') ++t;
        if (c == ' ' ) ++b;
    printf("\nNline:\t%d\nTab:\t%d\nBlank:\t%d\n", nl, t, b);
}