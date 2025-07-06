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
            // TODO: Make a filtering system like -fmt for formats, and it would be formatted this
            // way: -fmt '.mp3 .mp4 .pdf .etcetra' so u can specify which kinds of file this
            // program can just target.
            // TODO: Quiet mode. so it can download stuff silently with no logging whatsoever.
            // TODO: a configuration file so I can know some stuff.
            // Where to store every format.
            // Example:
            //          - Store .mp3 -> SOME_PATH
            //          - Store .mp4 -> SOME_OTHER_PATH
            // make a service of some sort to just kick start and stuff...
            // make a minimal notification system. needs to be very light weight and works as
            // intended. a very minimal program to call and notify me when everything has been
            // downloaded with success..
            // So I dont keep on waiting.
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
