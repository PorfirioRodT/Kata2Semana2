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

pub fn hextorgbpart2(color: &Color) -> Result<String, Error>{

    let hex_vecRGB = handle_hex_value(&color)?;

    let mut rgb_string = String::new();

    let rgb_toarray: [usize; 3] = [0, 2, 4];

    for x in rgb_toarray.iter() {

        let data = map::map_hex(hex_vecRGB[*x])* 16 + map::map_hex(hex_vecRGB[*x + 1]);

        rgb_string.push_str(&format!("{}", data));
        rgb_string.push(',');
        
    }

    if color.alpha{

        rgb_string.insert_str(0, "rgba(");
        if hex_vecRGB.len() == 0 {

            let data = (map::map_hex(hex_vecRGB[6])* 16 + map::map_hex(hex_vecRGB[7])) as f32 / 255f32;;

            rgb_string.push_str(&format!("{})", utils::round(data, 2)));
            
        } else {

            rgb_string.push_str("1)")
            
        }

    }else {
        
        rgb_string.insert_str(0, "rgb(");
        rgb_string.pop();
        rgb_string.push_str(")");

    }

    if color.upper {

        Ok(rgb_string.to_uppercase());
        
    }else {
        
        Ok(rgb_string.to_lowercase())

    }

}