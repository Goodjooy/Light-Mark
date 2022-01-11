
#[derive(Debug,serde::Serialize)]
pub struct Plain {
    pub inner:String
}
#[derive(Debug,serde::Serialize)]
pub struct Color {
    pub r:u8,
    pub g:u8,
    pub b:u8
}
#[derive(Debug,serde::Serialize)]
pub struct FontSize {
    pub size:f64
}
#[derive(Debug,serde::Serialize)]
#[serde(tag="type",content ="inner")]
pub enum Syntax {
    Plain(String),
    Color{color:Color,inner:Vec<Syntax>},
    FontSize{font_size:FontSize,inner:Vec<Syntax>},
    Url{name:Option<Vec<Syntax>>,url:String},
    Image{name:Option<String>,url:String},
    Blod(Vec<Syntax>),
    Italic(Vec<Syntax>),
    Paragraph,
    Seperate
}

impl From<String> for Syntax {
    fn from(s: String) -> Self {
        Self::Plain(s)
    }
}

impl From<Plain> for Syntax {
    fn from(s: Plain) -> Self {
        Self::Plain(s.inner)
    }
}

impl From<(Color,Vec<Syntax>)> for Syntax {
    fn from((c,s): (Color,Vec<Syntax>)) -> Self {
        Self::Color{ color: c, inner: s }
    }
}


impl From<(FontSize,Vec<Syntax>)> for Syntax {
    fn from((f,s): (FontSize,Vec<Syntax>)) -> Self {
        Self::FontSize{ font_size: f, inner: s }
    }
}

