#include <stdio.h>
#include <stdlib.h>
#include <stdbool.h>

const int WIDTH  = 100;
const int HEIGHT = 30;

struct animal {
    bool alive;
    int x, y, energy, dir;
    int genes[8];
};

struct plant {
	int x, y;
};

void
init_animal(struct animal *a)
{
    int i;
    
    a->x = WIDTH / 2;
    a->y = HEIGHT / 2;
    a->energy = 1000;
    a->dir = 0;
    a->alive = true;
    
    for(i = 0; i < 8; i++) {
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
    a->x = (a->x + ((a->dir >= 2 && a->dir <  5) ? 1 :
                    (a->dir == 1 || a->dir == 5) ? 0 : 
                                                  -1) + w) % w;
                                          
    a->y = (a->y + ((a->dir >= 0 && a->dir < 3) ? -1 :
                    (a->dir >= 4 || a->dir < 7) ?  1 : 
                                                   0) + h) % h;
    
    a->energy -= 1;
}

void
turn_animal(struct animal *a)
{
   a->dir += (a->genes[(a->x + a->y) % 8] > 5) ? 1 : -1; 
   a->dir = abs(a->dir);
}

void
eat_animal(struct animal *a, struct plant *p[])
{
    int i;

    for (i = 0; i < sizeof(p); i++) {
	 if (a->x == p[i]->x && a->y == p[i]->y) {
	      p[i] == 0;
     	 }
    }	 
}

int main() 
{
    struct animal animal_a;
    struct plant plants[WIDTH * HEIGHT];
    
    init_animal(&animal_a);
    
    if(animal_a.alive) show_animal(&animal_a);
    
    move_animal(&animal_a, WIDTH, HEIGHT);
    printf("\n");
    
    if(animal_a.alive) show_animal(&animal_a);
    printf("\n");

    turn_animal(&animal_a);
    if(animal_a.alive) show_animal(&animal_a);
    
    return 0;
}
