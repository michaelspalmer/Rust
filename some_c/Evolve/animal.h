typedef struct {
    bool alive;
    int x, y, energy, dir;
    int genes[8];
} animal;

//this shouldnt be here
typedef struct {
	int x, y;
} plant;

void show_animal(animal *a);
void move_animal(animal *a, int w, int h);
void turn_animal(animal *a);
void eat_animal(animal *a, plant *p[]);
bool reproduce_animal(animal *a, int rep_energy);
void still_alive_animal(animal *a);
animal copy_animal(animal *a);
void init_animal(animal *a);

