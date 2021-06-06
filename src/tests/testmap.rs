extern crate color_convert;

#[cfg(test)]
mod tests {

    use color_convert::handles::map;

    #[test]
    fn test_map_name(){

        let color = "White";
        let toColor = map::map_color_name(&color);
        assert_eq!(toColor, "#FFFFFF")

    }

    #[test]
    fn test_map_name_upper(){

        let color = "white";
        let toColor = map::map_color_upper(&color);
        assert_eq!(toColor, "#WHITE")

    }

    #[test]
    fn test_map_name_hex(){

        let color = "B";
        let toColor = map::map_color_hex(&color);
        assert_eq!(toColor, 11)

    }


}