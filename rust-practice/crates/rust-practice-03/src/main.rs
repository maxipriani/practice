// ------------------------------------------------------------------
// Función A: MUTABLE BORROW (Préstamo para Escritura)
// Recibe: &mut String (Referencia Exclusiva)
// 's' se crea al invocar modificar_buffer(), es una referencia mutable que apunta a un String
// Semántica: "Préstame el dato, lo voy a modificar, y te lo devuelvo".
// ------------------------------------------------------------------
fn modificar_buffer(s: &mut String) {
    // s.push_str(" [MODIFICADO]"); // Desreferencia automática: No hace falta (*s).push_str...
    *s = "[MODIFICADO]".to_string()
}

// ------------------------------------------------------------------
// Función B: IMMUTABLE BORROW (Préstamo para Lectura)
// Recibe: &String (Referencia Compartida)
// ------------------------------------------------------------------
fn leer_buffer(s: &String) {
    println!("Leyendo: {}", s);
}

fn main() {
    // 1. MUTABLE OWNER
    // Para poder prestar algo como mutable, el dueño debe permitirlo ('mut').
    let mut s_data = String::from("Dato Inicial");

    // --------------------------------------------------------------
    // 2. PRÉSTAMO MUTABLE EXITOSO
    // --------------------------------------------------------------
    // Creamos una referencia mutable.
    // REGLA: Mientras 'ref_mut' viva, 's_data' está "bloqueada" para todos los demás.
    let ref_mut = &mut s_data;
    
    modificar_buffer(ref_mut); 
    // Aquí 'ref_mut' se usa por última vez.
    // El compilador es inteligente: sabe que el "bloqueo" termina aquí (Non-Lexical Lifetimes).

    println!("Vuelta al dueño: {}", s_data); // s_data vuelve a estar accesible.

    // --------------------------------------------------------------
    // 3. EL CONFLICTO (La Regla de Oro)
    // --------------------------------------------------------------
    // Intentemos crear una referencia de lectura Y una de escritura simultáneas.
    
    let r1 = &s_data;      // OK: Préstamo de lectura (Reader Lock)
    let r2 = &s_data;      // OK: Otro préstamo de lectura. (Aliasing permitido)
    
    // SI DESCOMENTAS ESTO -> ERROR DE COMPILACIÓN
    // let w1 = &mut s_data; // ERROR: "cannot borrow `s_data` as mutable because it is also borrowed as immutable"
    
    println!("Lectores: {} y {}", r1, r2);
    // Aquí termina el uso de r1 y r2. El "Reader Lock" se libera.

    // --------------------------------------------------------------
    // 4. SOLUCIÓN CON SCOPES (Ámbitos)
    // --------------------------------------------------------------
    // Como Ingeniero de Sistemas, piensa en esto como una "Sección Crítica"
    {
        let w2 = &mut s_data; // Writer Lock adquirido
        w2.push_str(" - Segunda vez");
        // w2 muere aquí. Writer Lock liberado.
    } // El scope fuerza la liberación del préstamo.

    // Ahora podemos volver a leer sin miedo a Data Races.
    leer_buffer(&s_data); 

} // Fin de main. s_data muere.
