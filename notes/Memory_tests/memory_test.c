#include <stdio.h>
#include <stdlib.h>

int main() {
    int *arr = NULL;
    int avanti = 0;

    for (int x = 0; x < 10; x++) {

        printf("Inserisci un numero: ");
        fflush(stdout);

        scanf("%d", &avanti);

        printf("Ciclo: %d - Valore: %d\n", x, avanti);
        fflush(stdout);

        arr = malloc(100000 * sizeof(int));

        if (arr == NULL) {
            printf("Errore allocazione memoria!\n");
            return 1;
        }

        free(arr);
        arr = NULL;
    }

    return 0;
}