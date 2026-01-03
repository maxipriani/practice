// ------------------------------------------------------------------
// Función que fuerza un Realloc
// Recibe: &mut String (En C sería: String* s_ptr)
// Es un puntero a la estructura del Stack, NO al buffer de datos.
// ------------------------------------------------------------------
fn modify_buffer(s: &mut String) {
    // Al hacer push, si falta capacidad:
    // 1. Pide nueva memoria (0x9000).
    // 2. Copia los datos.
    // 3. Libera la vieja (0x1000).
    // 4. ACTUALIZA s.ptr en el Stack del llamador (main) para apuntar a 0x9000.
    s.push_str(" World!")
}

fn main() {
    // --------------------------------------------------------------
    // 1. ALLOCATION (Estructura en Stack + Buffer en Heap)
    // --------------------------------------------------------------
    // 's' es un struct en el Stack (digamos, dir 0xA0) con:
    //    { ptr: 0x1000, len: 5, cap: 5 }
    // El Heap en 0x1000 contiene: ['H','e','l','l','o']
    let mut s = String::from("Hello");

    // --------------------------------------------------------------
    // 2. MUTABLE BORROW (Indirección al Stack)
    // --------------------------------------------------------------
    // Creamos 'r'. 'r' NO apunta al Heap (0x1000).
    // 'r' apunta a la dirección de 's' en el Stack (0xA0).
    // Semántica: "Tengo control exclusivo sobre los metadatos de s".
    // ESTADO: 's' queda bloqueada. Nadie más puede mirar sus metadatos (ptr/len).
    let r = &mut s; 

    // --------------------------------------------------------------
    // 3. EL ERROR (Detección de Riesgo de Puntero Colgante)
    // --------------------------------------------------------------
    // Intentas leer 's'. Internamente 'println!' intentaría leer s.ptr (0x1000).
    // EL COMPILADOR RAZONA:
    // "Existe 'r' activo. 'r' tiene permiso para cambiar s.ptr (realloc) en cualquier momento.
    //  Si dejo que println lea 0x1000 ahora, y 'r' lo cambia a 0x9000 y libera 0x1000...
    //  println leería memoria liberada (Use-After-Free)."
    // 
    // Por seguridad, Rust prohíbe leer 's' mientras 'r' exista.
    // println!("s: {}", s); // <--- ERROR DE COMPILACIÓN
    
    // --------------------------------------------------------------
    // 4. MODIFICACIÓN (El Realloc ocurre aquí)
    // --------------------------------------------------------------
    // Pasamos 'r' (0xA0) a la función.
    // La función detecta falta de espacio y hace realloc.
    // Gracias a 'r', la función actualiza 's' en el Stack original.
    // Ahora 's' en 0xA0 es: { ptr: 0x9000, len: 12, ... }
    modify_buffer(r); 

    // --------------------------------------------------------------
    // 5. FIN DEL BLOQUEO
    // --------------------------------------------------------------
    // 'r' ya no se usa (el compiler lo sabe gracias a NLL). El bloqueo sobre 's' se libera.
    
    // Ahora es seguro leer 's'.
    // Leeremos el NUEVO puntero (0x9000) que dejó modify_buffer.
    println!("s: {}", s); 
}

