#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

const int WIDTH  = 100;
const int HEIGHT = 30;
const int REPNRG = 200;
// const int animals[WIDTH][HEIGHT];
// const plant plants[];

#include "animal.h

int main() 
{
    srand(time(NULL));
    
    animal animal_a;
    animal animal_b;
    // plant plants[WIDTH * HEIGHT];
    
    init_animal(&animal_a);
    animal_b = copy_animal(&animal_a);
    
    if(animal_a.alive) show_animal(&animal_a);
    printf("\n");
    if(animal_b.alive) show_animal(&animal_b);
    
    return 0;
}
