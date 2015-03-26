#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>
#include <time.h>

const int WIDTH  = 100;
const int HEIGHT = 30;
const int REPNRG = 200;
// const int animals[WIDTH][HEIGHT];
// const plant plants[];

typedef struct {
    bool alive;
    int x, y, energy, dir;
    int genes[8];
} animal;

//this shouldnt be here
typedef struct {
	int x, y;
} plant;

void
gen_genes(animal *a)
{
    int i;
    
    for(i = 0; i < 8; i++) {
        a->genes[i] = rand() % 10;
    }
}

void
mutate_gene(animal *a)
{
    int index = rand()% 8;
    int mutation = rand() % 3;
    a->genes[index] += mutation;
}

void
show_animal(animal *a)
{
    int i;
    
    printf("x: %d, y: %d\nenergy: %d\ndir: %d\nliving: %s\nGenes: ",
        a->x, a->y, a->energy, a->dir, (a->alive == 1) ? "true" : "false");
    
    for (i = 0; i < 8; i++) {
        printf("%d ", a->genes[i]);
    }
    printf("\n");
}

void
move_animal(animal *a, int w, int h)
{
    a->x = (a->x + ((a->dir >= 2 && a->dir <  5) ? 1 :
                    (a->dir == 1 || a->dir == 5) ? 0 : 
                                                  -1) + w) % w;
                                          
    a->y = (a->y + ((a->dir >= 0 && a->dir < 3) ? -1 :
                    (a->dir >= 4 || a->dir < 7) ?  1 : 
                                                   0) + h) % h;
    
    a->energy -= 1;
}

void
turn_animal(animal *a)
{
   a->dir += (a->genes[(a->x + a->y) % 8] > 5) ? 1 : -1; 
   a->dir = abs(a->dir);
}

//write this with a plain array
void
eat_animal(animal *a, plant *p[])
{
    int i;

    for (i = 0; i < sizeof(p); i++) {
	 if (a->x == p[i]->x && a->y == p[i]->y) p[i] = 0;
    }	 
}

bool
reproduce_animal(animal *a, int rep_energy)
{
    if (a->energy > rep_energy) {
        a->energy /= 2;
        return true;
    }
    return false;
}

void
still_alive_animal(animal *a)
{
    if (a->energy < 1) a->alive = false;
}

animal
copy_animal(animal *a)
{
    animal b;
    
    b.x = a->x;
    b.y = a->y;
    b.energy = a->energy;
    b.dir = (a->dir + 1) % 8;
    // b.genes = copy_genes(&a->genes);
    gen_genes(&b);
    b.alive = true;
    mutate_gene(&b);
    
    return b;
}

void
init_animal(animal *a)
{
    a->x = WIDTH / 2;
    a->y = HEIGHT / 2;
    a->energy = 1000;
    a->dir = 0;
    a->alive = true;
    gen_genes(a);
}

int main() 
{
    srand(time(NULL));
    
    printf("changed");
    
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