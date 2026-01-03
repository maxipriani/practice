// ------------------------------------------------------------------
// Función 1: BORROWING (Préstamo)
// Recibe una referencia (&String).
// Semántica: "Solo quiero leer tus datos, no me hago cargo de ellos".
// ------------------------------------------------------------------
fn foo(s: &String) { 
    // 's' es un puntero en el Stack que apunta a los datos del String original.
    // NO tiene ownership. NO es dueño de la memoria en el Heap.
    println!("foo (borrow): {}", s); 
} // Al terminar el ámbito: 's' desaparece del Stack, pero NO llama a drop().
  // drop() se llama solo para el Dueño (Owner) del recurso.
  // La memoria del Heap sigue intacta y pertenece a quien llamó a la función.

// ------------------------------------------------------------------
// Función 2: MOVE (Transferencia) 
// Firma: QUIERO OWNERSHIP (Dame la escritura de la casa)
// Recibe el valor por valor (String).
// Semántica: "Dame tus datos, ahora son míos y yo los limpiaré".
// La función toma ownership y hace drop()
// ------------------------------------------------------------------
fn foo_2(s: String){
    println!("foo_2 (move): {}", s); 
} // Al terminar el ámbito: 's' es dueña de los datos.
  // Rust inserta aquí una llamada automática a drop(s).
  // La memoria en el Heap se LIBERA.

fn main() {
    // 1. ALLOCATION
    // Se pide memoria en el Heap. s_1 es el único dueño (Owner).
    let s_1 = String::from("Hello World!");
    
    // --------------------------------------------------------------
    // 2. MOVE (Transferencia de Ownership local)
    // --------------------------------------------------------------
    // Rust hace una copia superficial (puntero, len, cap) de s_1 a s_2.
    let s_2 = s_1; 
    
    // ESTADO ACTUAL:
    // - s_2: Es el nuevo dueño activo.
    // - s_1: Queda INVALIDADA. (No es null, simplemente el compilador prohíbe su uso).
    // - Memoria Heap: Sigue en el mismo lugar, nadie la ha tocado ni liberado.

    // Si descomentas esto, error de compilación: "Use of moved value: s_1" 
    // foo(&s_1); 
    
    // --------------------------------------------------------------
    // 3. BORROW (Préstamo a función)
    // --------------------------------------------------------------
    // Le pasamos una referencia de s_2 a foo.
    foo(&s_2); 
    // Al volver de foo, s_2 sigue siendo dueña y sigue siendo válida.
    // Podemos seguir usándola.

    // --------------------------------------------------------------
    // 4. MOVE TO FUNCTION (Transferencia de Ownership a función)
    // --------------------------------------------------------------
    // Pasamos s_2 por valor. El ownership se mueve al argumento 's' de foo_2.
    foo_2(s_2);
    
    // CRÍTICO:
    // foo_2 terminó de ejecutarse y, como era dueña, liberó (drop) la memoria.
    // Ahora s_2 en main está INVALIDADA, igual que le pasó a s_1 antes.

    // --------------------------------------------------------------
    // 5. INTENTO DE USO POST-MORTEM
    // --------------------------------------------------------------
    // Si descomentas esto, error de compilación: "Use of moved value: s_2"
    // foo(&s_2); 

} // Fin de main.
