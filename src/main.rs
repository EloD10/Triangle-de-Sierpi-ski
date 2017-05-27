extern crate image; //Une librairie de traitement d'image
extern crate rand;

use rand::Rng;
use std::fs::File;

//Initialise nos points qio son servir à dessiner notre tringle
pub struct Point {    
    x: u32,
    y: u32,
}

pub const WIDTH:  u32 = 800; // largeur de l'image
pub const HEIGHT: u32 = 600; // hauteur de l'image


pub fn main() {
    let mut img = image::ImageBuffer::from_fn(WIDTH, HEIGHT, |x, y| {
        if x == 0 && y == 0 {
            image::Luma([0u8])      /*Donne une couleur, 0 pour noir, 255 pour blanc*/
        } else {
            image::Luma([255u8])
        }
    });

    let mut cnt: u32 = 1_000_000; /* Cette variable est le counter du nombre de points sur l'image, ce la nous permettre */

    let pts: [Point; 3] = [  /* Tabelau de type Point avec 3 index : dessine les point par rapport au à la largeur  et la hauteur de l'image défini comme constantes*/
        Point {x: WIDTH / 2, y: 0}, // Point en haut et au milieu du rectangle
        Point {x: 0, y: HEIGHT},    // Point bas à gauche 
        Point {x: WIDTH, y: HEIGHT}, // Point en bas à droite
    ];
    
    let mut p = Point { x: rand::thread_rng().gen_range(0, WIDTH),
                        y: rand::thread_rng().gen_range(0, HEIGHT),
    };

    let pixel = img[(0, 0)];
    while cnt > 0 {   /*tant que counter et supérieur à zéro on soustrait -1 et dessinera notre triangle, il y aura donc 999_999_999*/
        cnt = cnt - 1;
        let num = rand::thread_rng().gen_range(0,3);
        p.x = (p.x + pts[num].x) / 2; //créé les points entre les pts[]
        p.y = (p.y + pts[num].y) / 2; //créé les points entre les pts[]
        img.put_pixel(p.x, p.y, pixel);
    }

    let ref mut fout = File::create("tri.png").unwrap(); /* La méthode unwrap() permet d'outrepasser la gestion des erreurs */
    let _ = image::ImageLuma8(img).save(fout, image::PNG);  /* Sauvegarde de l'image en PNG*/
}