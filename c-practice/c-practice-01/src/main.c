#include <stdio.h>
#include <stdlib.h>
#include <string.h>

// ------------------------------------------------------------------
// Función 1: BORROWING (Simulado)
// Recibe un puntero char*.
// Contrato Social: "Prometo no llamar a free() sobre esto".
// ------------------------------------------------------------------
void foo_borrow(const char* s) {
    // PROTECCIÓN MANUAL: Verificar NULL porque C no garantiza referencias válidas.
    if (s == NULL) {
        printf("Error: Intento de borrow de un valor nulo\n");
        return;
    }
    printf("foo (borrow): %s\n", s);
}
// Al terminar: s desaparece del stack.
// No hay destructor. El heap queda intacto.

// ------------------------------------------------------------------
// Función 2: MOVE (Simulado)
// Recibe un puntero char*.
// Contrato Social: "Asumo que ahora soy dueño y debo liberar la memoria".
// ------------------------------------------------------------------
void foo_move(char* s) {
    if (s == NULL) return; // Protección básica

    printf("foo_2 (move): %s\n", s);

    // MANUAL DROP:
    // En Rust esto es automático. En C, si olvidas esto, es un MEMORY LEAK.
    free(s);
}
// Al terminar: La memoria ya fue liberada.
// Pero la variable 's' del main sigue teniendo la dirección antigua (Dangling Pointer).

int main(void) {
    // 1. ALLOCATION
    // malloc(13) reserva memoria en Heap.
    // s_1 es un simple puntero apuntando ahí.
    char* s_1 = malloc(13 * sizeof(char));
    if (s_1 == NULL) return 1;
    strcpy(s_1, "Hello World!");

    // --------------------------------------------------------------
    // 2. MOVE (Simulación manual de Transferencia)
    // --------------------------------------------------------------
    // Copiamos la dirección de memoria.
    // Ahora s_1 y s_2 apuntan AL MISMO LUGAR (Aliasing).
    char* s_2 = s_1;

    // RIESGO DE C (Falta de protección):
    // El compilador NO invalida s_1. Podrías seguir usándolo.
    // printf("%s", s_1); // <--- ESTO FUNCIONARÍA EN C (y es peligroso)

    // PROTECCIÓN MANUAL (Lo que Rust hace por ti):
    // Debemos anular manualmente s_1 para evitar usarlo por error ("Use after move").
    s_1 = NULL;

    // --------------------------------------------------------------
    // 3. BORROW
    // --------------------------------------------------------------
    // Pasamos el puntero. La función promete no liberarlo.
    foo_borrow(s_2);

    // --------------------------------------------------------------
    // 4. MOVE TO FUNCTION
    // --------------------------------------------------------------
    // Pasamos el puntero a una función que hará free().
    foo_move(s_2);

    // RIESGO CRÍTICO DE C:
    // foo_move liberó la memoria. Pero s_2 sigue guardando la dirección `0x1234`.
    // Esto es un "Dangling Pointer".

    // --------------------------------------------------------------
    // 5. INTENTO DE USO POST-MORTEM (Undefined Behavior)
    // --------------------------------------------------------------
    // En Rust, el compilador detiene esto.
    // En C, esto compila, corre, y probablemente cause un SEGFAULT o imprima basura.

    // foo_borrow(s_2); // <--- ¡PELIGRO! Use-After-Free

    // PROTECCIÓN MANUAL:
    // Deberíamos haber hecho esto inmediatamente después de llamar a foo_move:
    s_2 = NULL;
}