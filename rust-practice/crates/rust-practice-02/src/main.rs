fn main() { // Inicio del Stack Frame de main
    
    let s_outer = String::from("s_outer");

    { // Inicio de un ámbito interno (Inner Scope)
        let s_inner = String::from("s_inner");
        
        println!("Dentro: {}", s_inner);
    
    } // <--- AQUÍ MUERE s_inner. 
      // El compilador insertó `drop(s_inner)` aquí.
      // La memoria heap de s_inner se libera YA.
      // El Stack Frame de main SIGUE VIVO.

    println!("Fuera: {}", s_outer);
    
    // println!("{}", s_inner); // ERROR: "not found in this scope"

} // <--- AQUÍ MUERE s_outer.
  // El compilador insertó `drop(s_outer)`.
  // Se destruye el Stack Frame de main.