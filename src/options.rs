#[allow(dead_code)]
#[derive(Debug)]
pub struct Options {
    pub use_youtube: bool,
    pub use_wget: bool,
    pub quiet: bool,
    pub download_path: String,
    // pub confirmation: bool
}

fn help(program_name: &String) {
    println!("Usage: {} [DOWNLOAD_PATH] [...Options]", program_name);
    println!("Options:");
    println!("    -h  | --help: Displays this menu");
    println!("    -fs | --fmts: specifies which format to care about when getting links to download");
    println!("          Example: {} . -fs \'.mp3 .mp4 .wav\'", program_name);
    println!("          Explaination: it will only download the formats given");
    println!("    -y  | --use_youtube: Use `yt-dlp` to download youtube videos");
    println!("    -w  | --use_wget: Use `wget` to download content");
    println!("    -q  | --quiet: be quiet lol");
}

fn format_help(program_name: &String, error: String)
{
    println!("{}", error);
    help(program_name);
}

#[allow(dead_code)]
impl Options
{
    pub fn new() -> Self
    {
        Self {
            use_youtube: false,
            use_wget: false,
            quiet: false,
            download_path: String::from("")
        }
    }

    pub fn parse_options(&mut self, opts: &Vec<String>) -> i32
    {
        let mut arg;
        let mut i = 2;

        self.download_path = String::from("DEFAULT");
        while i < opts.len() {
            arg = opts[i].clone();
            match &arg as &str {
                "-y" | "--use_youtube" => {
                    self.use_youtube = true;
                },
                "-w" | "--use_wget" => {
                    self.use_wget = true;
                },
                "-h" | "--help" => {
                    help(&opts[0]);
                    return 0;
                },
                "-fs" | "--fmts" => {
                    i = i + 1;
                    if i >= opts.len() {
                        format_help(&opts[0], format!("-fs: a format list was not provided, please provide the list").clone());
                        return 0;
                    }
                    // arg = opts[i].clone();
                },
                "-q" | "--quiet" => {
                    self.quiet = true;
                },
                _ => {
                    println!("Invalid Options: {}", arg);
                    return 0;
                }
            }
            // TODO: a configuration file so I can know some stuff. S
            // Where to store every format.
            // Example:
            //          - Store .mp3 -> SOME_PATH
            //          - Store .mp4 -> SOME_OTHER_PATH
            // make a service of some sort to just kick start and stuff...
            // make a minimal notification system. needs to be very light weight and works as
            // intended. a very minimal program to call and notify me when everything has been
            // downloaded with success..
            // So I dont keep on waiting.
            i += 1;
        }
        if !self.use_youtube && !self.use_wget
        {
            self.use_wget = true;
            self.use_youtube = true;
        }
        if opts.len() > 1
        {
            self.download_path = opts[1].clone();
        }
        return 1;
    }
    pub fn log(&mut self)
    {
        println!("use_wget: {}", self.use_wget);
        println!("use_youtube: {}", self.use_youtube);
        println!("download_path: {}", self.download_path);
        println!("quiet: {}", self.quiet);
    }
}
