
#[allow(dead_code)]
#[derive(Debug)]
pub struct Options {
   pub use_youtube: bool,
   pub use_wget: bool,
   pub download_path: String
}

#[allow(dead_code)]
impl Options
{
    pub fn new() -> Self
    {
        Self {
            use_youtube: false,
            use_wget: false,
            download_path: String::from("")
        }
    }
    pub fn parse_options(&mut self, opts: &Vec<String>)
    {
        self.download_path = String::from("DEFAULT");
        if opts.len() > 1
        {
            self.download_path = opts[1].clone();
        }
        for arg in opts {
            if arg == "-y" || arg == "--use_youtube" {
                self.use_youtube = true;
            }
            if arg == "-w" || arg == "--use_wget" {
                self.use_wget = true;
            }
        }
        if !self.use_youtube && !self.use_wget
        {
            self.use_wget = true;
            self.use_youtube = true;
        }
    }
    pub fn log(&mut self)
    {
        println!("use_wget: {}", self.use_wget);
        println!("use_youtube: {}", self.use_youtube);
        println!("download_path: {}", self.download_path);
    }
}
