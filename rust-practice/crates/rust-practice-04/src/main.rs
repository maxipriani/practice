fn modify_buffer(s: &mut String) {
    s.push_str(" World!")
}

fn main() {
    // 1. ALLOCATION
    let mut s = String::from("Hello");
    
    // --------------------------------------------------------------
    // 2. INICIO DEL BLOQUEO (Start of Lifetime)
    // --------------------------------------------------------------
    // Se crea 'r'. 
    // Desde esta línea, 's' queda secuestrada por 'r'.
    // El compilador se pregunta: "¿Hasta dónde tengo que mantener vivo a 'r'?"
    let r = &mut s; 
    
    // --------------------------------------------------------------
    // 3. USO INTERMEDIO
    // --------------------------------------------------------------
    // Aquí usamos 'r'. Todo bien.
    // Si el programa terminara aquí (o 'r' no se usara más), el bloqueo se liberaría
    // inmediatamente después de esta línea (gracias a NLL).
    modify_buffer(r);

    // --------------------------------------------------------------
    // 4. ZONA DE CONFLICTO (El Error)
    // --------------------------------------------------------------
    // Intentas leer 's'.
    // El compilador mira hacia abajo y ve la línea "CULPABLE" (paso 5).
    // Como 'r' se usa más abajo, el compilador dice:
    // "El puntero 'r' sigue activo y tiene permiso de escritura (*) (realloc)".
    // "Por tanto, NO puedo permitirte leer 's' aquí todavía".
    
    // ERROR: cannot borrow `s` as immutable because it is also borrowed as mutable
    // println!("s: {}", s); // <---  ESTO EXPLOTA
    
    // --------------------------------------------------------------
    // 5. EL CULPABLE (Lifetime Extender)
    // --------------------------------------------------------------
    // Esta es la línea crítica. 
    // Como 'r' se usa AQUÍ por última vez, el "tiempo de vida" (scope) de 'r'
    // se estira obligatoriamente desde la línea 2 hasta la línea 5.
    // Esto hace que la línea 4 caiga DENTRO de la zona de bloqueo exclusivo.
    modify_buffer(r);

} // Fin de main. Aquí muere 'r' definitivamente.

/*
    (*)

    - Es importante la distinción: "'r' sigue activo y tiene permiso de -ESCRITURA- (realloc)".
    - Todo el drama y los errores de compilación que vimos antes son culpa de la EXCLUSIVIDAD que exige &mut.
    - Si r fuera una referencia inmutable (&), Rust relaja las reglas completamente y permite que convivan "muchos lectores" al mismo tiempo.
    - Esto equivale a un RWLock (Read-Write Lock) en compile-time:
        - 1. Caso &mut (Write Lock): Solo UNO puede tener la llave. Nadie más puede leer ni escribir. (El caso que fallaba).
        - 2. Caso & (Read Lock): INFINITOS procesos pueden tener la llave para leer. Pero NADIE puede escribir.
        
    === La Regla de los Punteros (Pointer Aliasing) === 
    - Rust tiene una regla estricta para garantizar la seguridad de la memoria:

        - Puedes tener UNO de los siguientes, pero NO ambos al mismo tiempo:
            - Cualquier número de referencias inmutables (&T).
            - Exactamente UNA referencia mutable (&mut T).

    - Cuando hacés let r = &mut s;, r adquiere un "candado de escritura" (bloqueo exclusivo) sobre s.

*/