#include <stdio.h>

// Macro de C11 para inspección de tipos en tiempo de compilación.
// Esto prueba qué "ve" el compilador antes de ejecutar nada.
#define TYPEOF(expr) _Generic((expr), \
    int:    "int",                \
    int*:   "int*",               \
    float*: "float*",             \
    void*:  "void*",              \
    default: "other")

int main() {
    // 1. Definimos un entero en el Stack.
    int x = 40;

    // -------------------------------------------------------------
    // CONCEPTO CLAVE: La Expresión Pura
    // -------------------------------------------------------------
    // La expresión '&x' se evalúa INMEDIATAMENTE a un tipo 'int*'.
    // No hace falta guardarlo en una variable 'p' (L-value) para que "sea" un puntero.
    // Es un valor temporal (R-value) que representa la dirección de memoria.
    
    &x; // Esta línea es válida. El compilador genera la dirección, 
        // ve que es un int*, y luego la descarta porque no la usamos.

    // Demostración de tipos (Tiempo de Compilación):
    printf("1. Tipo de 'x':  %s\n", TYPEOF(x));  // Es un int
    printf("2. Tipo de '&x': %s\n", TYPEOF(&x)); // ¡Ya es un int*!

    // Mostramos el valor del puntero temporal generado por la expresión
    printf("3. Valor de '&x' (Direccion): %p\n", (void*)&x);

    printf("--------------------------------------\n");

    // -------------------------------------------------------------
    // CONCEPTO SECUNDARIO: La Variable Puntero (Storage)
    // -------------------------------------------------------------
    // Aquí solo estamos COPIANDO el valor (la dirección) que generó '&x'
    // dentro de un espacio reservado en el Stack llamado 'p'.
    
    int* p = &x; 

    // 'p' tiene el mismo VALOR que '&x'
    printf("4. Valor guardado en 'p':     %p\n", (void*)p);

    // Pero 'p' tiene su PROPIA dirección (es una variable distinta)
    printf("5. Direccion de la variable 'p': %p\n", (void*)&p);

    return 0;
}