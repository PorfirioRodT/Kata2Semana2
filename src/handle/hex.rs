use color::{Color, Error};
use handle::map;

pub fn handle_hex_value<'a>(color: &'a Color) -> Result<Vec<&'a str>, Error> {

    let mut hex_vec: Vec<&'a str> = color.to_str().split("").collect();

    hex_vec.retain(|&x| x != "" && x != "#");

    let mut return_vex: Vec<&'a str> = vec![];

    match hex_vec.len() {

        3 => {

            for elem in &hex_vec {

                return_vex.extend_from_slice(&[*elem, *elem]);
                
            }

        },
        
        6 => return_vex.extend(&hex_vec),

        8 => {

            if color.android {

                return_vex.extend(&hex_vec[2..]);

                return_vex.extend(&hex_vec[0..2]);
                
            }

        },

        _ => return Err(Error::Format)

    }
    Ok(return_vex)
    
}