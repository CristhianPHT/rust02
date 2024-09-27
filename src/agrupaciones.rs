/* 1. Arrays ([T; N])
Un array en Rust tiene un tamaño fijo y todos los elementos deben ser del mismo tipo.
El tamaño del array es parte de su tipo,
lo que significa que un array de 5 elementos es diferente de uno de 10
*/
let arr: [i32; 3] = [1, 2, 3];

/* 2. Vectors (Vec<T>)
Un vector es una colección dinámica, similar a las listas en Python. Puede crecer y reducir su tamaño.
Es el tipo de colección más común en Rust para listas dinámicas.
*/
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
// O usando el macro vec!:
let vec = vec![1, 2, 3];

/* 3. Tuplas ((T1, T2, ..., Tn))
Una tupla puede contener varios valores de diferentes tipos y no tiene un tamaño fijo.
Los elementos se acceden mediante índices.
*/
let tuple = (1, "hola", 3.5);
println!("{}", tuple.1); // "hola"

/* 4. HashMaps (HashMap<K, V>)
Un HashMap es una colección clave-valor, similar a los diccionarios en Python.
Las claves deben ser del mismo tipo y los valores también, pero los tipos de clave y valor pueden ser diferentes entre sí.
Necesitas importar la colección antes de usarla.
*/
use std::collections::HashMap;
let mut map = HashMap::new();
map.insert("clave1", 10);
map.insert("clave2", 20);

/* 5. BTreeMap
Similar a HashMap, pero los elementos se mantienen ordenados según la clave.
*/
use std::collections::BTreeMap;
let mut map = BTreeMap::new();
map.insert("clave1", 10);
map.insert("clave2", 20);

/* 6. HashSet (HashSet<T>)
Similar a los conjuntos en Python, un HashSet es una colección que contiene elementos únicos sin un orden específico.
También requiere ser importado.
*/
use std::collections::HashSet;
let mut set = HashSet::new();
set.insert(1);
set.insert(2);
set.insert(2); // No se agregará de nuevo

/* 7. BTreeSet
Similar a HashSet, pero mantiene los elementos en orden.
*/
use std::collections::BTreeSet;
let mut set = BTreeSet::new();
set.insert(3);
set.insert(1);
set.insert(2);

/* 8. Option (Option<T>)
Option es un tipo de dato que puede representar un valor presente:
(Some(T)) o ausente (None), similar a None en Python.
*/
let valor: Option<i32> = Some(5);
let sin_valor: Option<i32> = None;

/* 9. Result (Result<T, E>)
Result es un tipo que se utiliza comúnmente para representar
el éxito (Ok(T)) o el error (Err(E)) en operaciones que pueden fallar.
*/
let result: Result<i32, String> = Ok(10);
let error: Result<i32, String> = Err("Algo salió mal".to_string());

/* 10. Slices (&[T])
Los "slices" son vistas sobre una parte de un array o vector, 
permitiendo acceder a una subsección sin copiar los datos.
*/
let arr = [1, 2, 3, 4];
let slice = &arr[1..3]; // Contiene [2, 3]

/* 11. Strings (String y &str)
String: Es una cadena mutable y asignada en heap (crece dinámicamente).
&str: Es una referencia inmutable a una cadena (es una vista).
*/
let s = String::from("Hola");
let s_ref: &str = "Hola Mundo"; // Vista inmutable

/*
*/