extern crate color_convert;
pub mod common;

#[cfg(test)]
mod tests {

    use color_convert::color::Color;
    use color_convert::handles::hex;
    use common::init_color;

    #[test]
    fn handle_hex() {

        let value = Color.init("#FFFFFF", true, true, true);

        let vec_value = hex::handle_hex_value(&value).unwrap();

        assert_eq!(vec!["F","F","F","F","F","F"],vec_value);

        //unimplemented!();

    }

    #[test]
    fn test_hextorgbpart2() {
        //unimplemented!();

        let value = vec!["#FFFFFF"];
        let value_result = vec![
            vec!["rgb(,,,)", "RGB(,,,)", "rgb(,,,)", "rgba(,,,,)",
                 "RGB(,,,)", "RGBA(,,,,)", "rgba(,,,,)", "RGBA(,,,,)"],
        ];

        let test_color = init_color(value);

        for (x, vec_color) in test_color.iter().enumerate(){

            for (index, color) in vec_color.iter().enurate(){

                assert_eq!(value_result[x][index], color.to_rgb().unwrap())
                
            }
            
        }

    }

}