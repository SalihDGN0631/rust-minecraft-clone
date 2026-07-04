

use macroquad::prelude::*;

struct Text {
    text : String,
    font_size : f32,
    color : Color,
    position : (f32,f32),
}

impl Text {

    fn new(_text:&str,_font_size:f32,color:Color,position:(f32,f32)) -> Text {

        Text {
            text : _text.to_string(),
            font_size : _font_size,
            color : color,
            position : position
        }

    }

    fn draw(&self) {

        draw_text(&self.text, self.position.0, self.position.1, self.font_size, self.color);

    }

    fn update_text(&mut self,text:&str) {

        self.text = text.to_string();

    }

}

struct Button {

    position : (f32,f32),
    text : Text,
    OnHoverCallback : (),
    OnClickCallback : (),
    OnMouseLeave : (),
    size : (f32,f32)


}

fn create_button_text(text:&str,pos:(f32,f32),color:Color,font_size:f32) -> Text {
        Text {
            text : text.to_string(),
            font_size : font_size,
            color : color,
            position : pos
        }


    }




impl Button {


    fn new(pos:(f32,f32),text:&str,size:(f32,f32)) -> Button {

        Button {

            position : pos,
            text : create_button_text(text, pos, BLACK, 20.0),
            OnHoverCallback : (),
            OnClickCallback : (),
            OnMouseLeave : (),
            size : size

        }

    }

    fn draw_button(&self) {

        draw_rectangle(self.position.0, self.position.1, self.size.0, self.position.1, WHITE);
        &self.text.draw();

    }

    
}




struct UI {

    
    


}



impl UI {


    fn new() -> UI {
        UI {

        }       
    }

    fn draw_UI(&self) {

    }
}