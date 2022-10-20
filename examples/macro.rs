// Macros Déclaratives  println!

// Macros Procédurales #derive[debug, clone etc...]

// Exemple de macro déclarative :

macro_rules! simple_vec {
    ( $( $x:expr ),* ) => { // expr => n'importe quelle  valeur de types numériques / char / string; * => 0 ou plus; ? => 0 ou 1
        // On passe 0 ou plus expr séparées par des ,
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )* // répété autant de fois qu'il y a de valeurs de x
            temp_vec // On retourne le vecteur
        }
    };

    // Add another arm so that we can initialize vec3 in main
    ($x:expr;$y:expr) => { 
        {
            let mut temp_vec = Vec::new();
            for i in 0..$y 
            {
                temp_vec.push($x);
            }
            temp_vec 
        }
    };
}


fn main () {
    let vec1:Vec<char> = simple_vec!('a', 'b', 'c');
    let vec2:Vec<i32> = simple_vec!(1, 2, 3);
    let vec3:Vec<&str> = simple_vec!("coucou"; 9); // vecteur avec 5 fois coucou

    println!("{:?}\n{:?}", vec1, vec2);
    println!("{:?}", vec3);


}