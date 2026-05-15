/*
1. ¿Qué ocurre con la propiedad (ownership) cuando se usa `Option::take()` dentro de las rotaciones?
take() transfiere la propiedad del nodo contenido en el `Option` hacia otra variable y deja `None` 
en su lugar. Esto permite mover nodos de forma segura sin violar las reglas de ownership de Rust 
ni provocar errores del Borrow Checker.

2. ¿Por qué usamos `Box<Nodo>` en lugar de `Nodo` directamente?
Porque un `Nodo` contiene otros nodos dentro de sí mismo, lo que produciría un tamaño 
infinito en memoria. `Box<Nodo>` guarda los datos en el heap y permite que el compilador 
conozca un tamaño fijo para la estructura durante la compilación.
 */


#[derive(Debug, Clone)]
struct Vuelo {
    id: String,
    altitud: u32, // Este será nuestra clave (key)
}

struct Nodo {
    vuelo: Vuelo,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

impl Nodo {
    fn nuevo(vuelo: Vuelo) -> Self {
        Nodo {
            vuelo,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

// --- UTILIDADES DE BALANCEO (NO MODIFICAR) ---

fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Error de radar");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Error de radar");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

// --- FUNCIÓN DE INSERCIÓN ---

fn insertar(nodo_opt: Option<Box<Nodo>>, vuelo: Vuelo) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(vuelo)),
        Some(n) => n,
    };

    if vuelo.altitud < nodo.vuelo.altitud {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), vuelo.clone()));
    } else if vuelo.altitud > nodo.vuelo.altitud {
        nodo.derecho = Some(insertar(nodo.derecho.take(), vuelo.clone()));
    } else {
        return nodo;
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    // Caso Izquierda-Izquierda
    if balance > 1 && vuelo.altitud < nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Derecha
    if balance < -1 && vuelo.altitud > nodo.derecho.as_ref().unwrap().vuelo.altitud {
        return rotar_izquierda(nodo);
    }
    // Caso Izquierda-Derecha
    if balance > 1 && vuelo.altitud > nodo.izquierdo.as_ref().unwrap().vuelo.altitud {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    // Caso Derecha-Izquierda
    if balance < -1 && vuelo.altitud < nodo.derecho.as_ref().unwrap().vuelo.altitud {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }

    nodo
}

fn buscar_vuelo(nodo: &Option<Box<Nodo>>, altitud: u32) -> Option<&Vuelo> {
    match nodo {
        None => None,
        Some(n) => {
            if altitud == n.vuelo.altitud {
                Some(&n.vuelo)
            } else if altitud < n.vuelo.altitud {
                buscar_vuelo(
                    &n.izquierdo,
                    altitud,
                )
            } else {
                buscar_vuelo(
                    &n.derecho,
                    altitud,
                )
            }
        }
    }
}

fn eliminar_vuelo(nodo_opt: Option<Box<Nodo>>, altitud: u32) -> Option<Box<Nodo>> {
    let mut nodo = match nodo_opt {
        None => return None,
        Some(n) => n,
    };

    if altitud < nodo.vuelo.altitud {
        nodo.izquierdo = eliminar_vuelo(
            nodo.izquierdo.take(),
            altitud,
        );
    } else if altitud > nodo.vuelo.altitud {
        nodo.derecho = eliminar_vuelo(
            nodo.derecho.take(),
            altitud,
        );
    } else {
        if nodo.izquierdo.is_none() {
            return nodo.derecho;
        }
        if nodo.derecho.is_none() {
            return nodo.izquierdo;
        }
        let predecesor = obtener_maximo(
            nodo.izquierdo
                .as_ref()
                .unwrap()
        );

        nodo.vuelo = predecesor.clone();

        nodo.izquierdo = eliminar_vuelo(
            nodo.izquierdo.take(),
            predecesor.altitud,
        );
    }

  
    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1
        && obtener_balance(
            nodo.izquierdo
                .as_ref()
                .unwrap()
        ) >= 0
    {
        return Some(rotar_derecha(nodo));
    }

    if balance > 1
        && obtener_balance(
            nodo.izquierdo
                .as_ref()
                .unwrap()
        ) < 0
    {
        let hijo_izq =
            nodo.izquierdo.take().unwrap();
        nodo.izquierdo =
            Some(rotar_izquierda(hijo_izq));
        return Some(rotar_derecha(nodo));
    }

    if balance < -1
        && obtener_balance(
            nodo.derecho
                .as_ref()
                .unwrap()
        ) <= 0
    {
        return Some(rotar_izquierda(nodo));
    }

    if balance < -1
        && obtener_balance(
            nodo.derecho
                .as_ref()
                .unwrap()
        ) > 0
    {
        let hijo_der =
            nodo.derecho.take().unwrap();
        nodo.derecho =
            Some(rotar_derecha(hijo_der));
        return Some(rotar_izquierda(nodo));
    }

    Some(nodo)
}

fn vuelos_en_rango(nodo: &Option<Box<Nodo>>, min: u32, max: u32) -> usize {
    match nodo {
        None => 0,
        Some(n) => {
            let mut count = 0;
            if n.vuelo.altitud >= min && n.vuelo.altitud <= max {
                count += 1;
            }
            if n.vuelo.altitud > min {
                count += vuelos_en_rango(&n.izquierdo, min, max);
            }
            if n.vuelo.altitud < max {
                count += vuelos_en_rango(&n.derecho, min, max);
            }
            count
        }
    }
}


fn obtener_maximo(nodo: &Box<Nodo>) -> Vuelo {
    let mut actual = nodo;
    while let Some(ref d) = actual.derecho {
        actual = d;
    }
    actual.vuelo.clone()
}

fn imprimir_inorder(nodo: &Option<Box<Nodo>>) {
    if let Some(n) = nodo {
        imprimir_inorder(&n.izquierdo);
        println!( "Vuelo: {} | Altitud: {} | Altura Nodo: {}", n.vuelo.id, n.vuelo.altitud, n.altura);
        imprimir_inorder(&n.derecho);
    }
}

fn main() {
    let mut radar: Option<Box<Nodo>> = None;

    // Simulación de entrada de vuelos
    let datos = vec![
        ("AV123", 5000), ("UA456", 3000), ("IB101", 2000),
        ("AF999", 4000), ("TA222", 3500), ("AM777", 6000),
    ];

    for (id, alt) in datos {
        let v = Vuelo { id: id.to_string(), altitud: alt };
        radar = Some(insertar(radar.take(), v));
    }

    println!("--- Radar de Control Aéreo (AVL) ---");
    // Aquí el estudiante debe invocar sus funciones de búsqueda y eliminación

    println!("\n====================================");
    println!("BÚSQUEDA DE VUELO");

    match buscar_vuelo(&radar, 3500) {
        Some(v) => println!("Vuelo encontrado -> ID: {} | Altitud: {}", v.id, v.altitud),
        None => println!("Vuelo no encontrado"),
    }

    println!("\n====================================");
    println!("ALERTA DE PROXIMIDAD");

    let cantidad = vuelos_en_rango(&radar, 3000, 5000);
    println!("Cantidad de vuelos entre 3000 y 5000 pies: {}", cantidad);


    println!("\n====================================");
    println!("ELIMINACIÓN DE VUELO");

    radar = eliminar_vuelo( radar.take(), 4000,);
    println!("Vuelo con altitud 4000 eliminado");

    println!("\n====================================");
    println!("RADAR AVL - ESTADO FINAL");
    imprimir_inorder(&radar);
}