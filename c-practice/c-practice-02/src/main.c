#include <stdio.h>

int main() {
    // ---------------------------------------------------------
    // ESTADO INICIAL
    // Hex: 0x28
    // Dec: 40
    // Bin: 0010 1000  <-- Fíjate en el '1' en la posición 32 (2^5) y 8 (2^3)
    // ---------------------------------------------------------
    int x = 0x28;
    printf("Inicio:     %d (Hex: 0x%X)\n", x, x);

    // ---------------------------------------------------------
    // RIGHT SHIFT (>>) - DIVISIÓN
    // Desplazar 1 bit a la derecha es dividir por 2^1 (2).
    // Binario anterior: 0010 1000 (40)
    //                     \    \ 
    // Nuevo Binario:    0001 0100 (20)
    // ---------------------------------------------------------
    x = x >> 1;
    // Matemáticamente: 40 / 2 = 20
    printf("x >> 1:     %d\n", x);

    // ---------------------------------------------------------
    // LEFT SHIFT (<<) - MULTIPLICACIÓN
    // Desplazar 3 bits a la izquierda es multiplicar por 2^3 (8).
    // Binario anterior: 0000 ... 0001 0100 (20)
    //                            /    /
    //                       <-- 3 espacios
    // Nuevo Binario:    0000 ... 1010 0000 (160)
    // ---------------------------------------------------------
    x = x << 3;
    // Matemáticamente: 20 * (2*2*2) = 20 * 8 = 160
    printf("x << 3:     %d\n", x);
}