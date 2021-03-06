extern crate color_convert;
use color_convert::color::Color;

pub fn init_color(color:Vec<&str>) -> Vec<Vec<Color>>{

    let mut test_color = vec![];

    for i in 0..Color.len(){

        let result = vec![];
        result.push(Color::init(color[i], false, false, false));
        result.push(Color::init(color[i], true, true, true));
        result.push(Color::init(color[i], true, false, false));
        result.push(Color::init(color[i], false, true, false));
        result.push(Color::init(color[i], false, false, true));
        result.push(Color::init(color[i], true, true, false));
        result.push(Color::init(color[i], true, false, true));
        result.push(Color::init(color[i], false, true, true));
        
        test_color.push(result);

    }
    test_color

}