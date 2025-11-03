#[allow(dead_code)]
#[derive(Debug)]
pub struct Options {
    pub use_youtube: bool,
    pub use_wget: bool,
    pub quiet: bool,
    pub download_path: String,
    pub download_path_set: bool,
    pub formats: Vec<String>
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
            download_path: String::from(""),
            formats: Vec::<String>::new(),
            download_path_set: false
        }
    }

    pub fn is_fmt_supported(&self, link: &str) -> bool {
        let link_lower = link.to_lowercase();
        self.formats.iter().any(|ext| link_lower.ends_with(&ext as &str))
    }

    pub fn parse_options(&mut self, opts: &Vec<String>) -> i32
    {
        let mut arg;
        let mut i = 1;
        let defualt_fmts = [
            ".mp3", ".wav", ".flac", ".aac", ".ogg", ".m4a", ".wma",
            ".mp4", ".avi", ".mkv", ".mov", ".wmv", ".flv", ".webm", ".m4v",
            ".jpg", ".jpeg", ".png", ".gif", ".bmp", ".tiff", ".svg", ".webp",
            ".pdf",
        ];

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
                        format_help(&opts[0], format!("{}: a format list was not provided, please provide the list", arg).clone());
                        return 0;
                    }
                    arg = opts[i].clone();
                    let fmts: Vec<String> = arg.split(' ')
                    .filter(|s| !s.is_empty())
                    .map(|s| s.to_string())
                    .collect();
                    if fmts.len() == 0 {
                        format_help(&opts[0], format!("{}: a format list was not provided, please provide the list>!<", opts[i - 1]).clone());
                        return 0;
                    }

                    fmts.iter().for_each(|s| self.formats.push(s.to_string()));
                },
                "-q" | "--quiet" => {
                    self.quiet = true;
                },
                _ => {
                    if i != 1 {
                        println!("Invalid Options: {}", arg);
                        return 0;
                    }
                }
            }
            i += 1;
        }
        if self.formats.len() == 0{
            defualt_fmts.iter().for_each(|s| self.formats.push(s.to_string()));
        }
        if !self.use_youtube && !self.use_wget
        {
            self.use_wget = true;
            self.use_youtube = true;
        }
        if opts.len() > 1
        {
            self.download_path = opts[1].clone();
            self.download_path_set = true;
        }
        return 1;
    }
    pub fn log(&mut self)
    {
        println!("use_wget: {}", self.use_wget);
        println!("use_youtube: {}", self.use_youtube);
        println!("download_path: {}", self.download_path);
        println!("quiet: {}", self.quiet);
        println!("formats:");
        for i in 0..self.formats.len()
        {
            print!("{}", self.formats[i]);
            if i < self.formats.len() - 1 {
                print!("|");
            }
        }
        print!("\n");
    }
}
