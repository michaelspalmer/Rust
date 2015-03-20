#include <stdio.h>
#include <stdbool.h>

struct animal {
    bool alive;
    int x, y, energy, dir;
    int genes[8];
};


void
init_animal(struct animal *a)
{
    int i;
    
    a->x = 0;
    a->y = 0;
    a->energy = 0;
    a->dir = 0;
    a->alive = true;
    
    for(i = 0; i < 3; i++) {
        a->genes[i] = i;
    }
}

void
show_animal(struct animal *a)
{
    printf("%d, %d, %d, %d, %d, %d, %d, %s\n",
        a->x, a->y, a->energy, a->dir,
        a->genes[0], a->genes[1], a->genes[2],
        (a->alive == 1) ? "true" : "false");
}

void
move_animal(struct animal *a, int w, int h)
{
    a->x = (a->x + (a->dir >= 2 && a->dir < 5) ?  1 :
                  ((a->dir == 1 || a->dir == 5) ? 0 : 
                                                 -1) + w) % w;
                                          
    a->y += ((a->dir >= 0 && a->dir < 3) ? -1 :
            ((a->dir >= 4 || a->dir < 7) ?  1 : 
                                            0) + h) % h;
    
    a->energy -= 1;
}

int main() 
{
    struct animal animal_a;
    
    init_animal(&animal_a);
    
    if(animal_a.alive) show_animal(&animal_a);
    
    move_animal(&animal_a, 100, 30);
    printf("\n");
    
    if(animal_a.alive) show_animal(&animal_a);
    
    return 0;
}