use clap::{Arg, Command};

// today we are doing argument parsing with Clap

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DevilFruit
{
    english_name: String,
    japanese_name: String
}

impl DevilFruit
{
    pub fn new(english_name: String, japanese_name: String) -> Self
    {
        DevilFruit
        {
            english_name,
            japanese_name
        }
    }
}

fn main()
{
    let matches = Command::new("Devil Fruit Generator")
        .version("1.0.0")
        .author("Sam Stockstrom <sstockburn96@gmail.com>")
        .about("Generates a random devil fruit from One Piece")
        .arg(Arg::new("lang")
                .short('l')
                .long("lang")
                .help("eng or jap for devil fruits (default: eng)"))
        .arg(Arg::new("count")
                .short('c')
                .long("count")
                .help("The amount of devil fruits you want (default: 1)"))
        .get_matches();

    let default_lang = "eng".to_string();
    let language = matches.get_one::<String>("lang").unwrap_or(&default_lang);
    let count_str = matches.get_one::<String>("count");
    let mut count: i32 = 1;
    match count_str
    {
        None => count = 1,
        Some(s) => 
        {
            match s.parse::<i32>() 
            {
                Ok(n) => count = n,
                Err(_) => println!("-c was not a valid number.\n{}", s)
            }
        }
    }

    println!("generating {} devil fruits in {} language...", count, language);

    // create devil fruit vector

    // data from here
    // https://comicvine.gamespot.com/profile/stmichalofwilson/lists/devil-fruit-list/62241/
    let mut fruits: Vec<DevilFruit> = vec![
        DevilFruit::new("Tremor-Tremor Fruit".to_string(), "Gura Gura no Mi".to_string()),
        DevilFruit::new("Cat-Cat Fruit: Model Leopard".to_string(), "Neko Neko no Mi".to_string()),
        DevilFruit::new("Flame-Flame Fruit".to_string(), "Mera Mera no Mi".to_string()),
        DevilFruit::new("Hollow-Hollow Fruit".to_string(), "Horo Horo no Mi".to_string()),
        DevilFruit::new("Rumble-Rumble Fruit".to_string(), "Goro Goro no Mi".to_string()),
        DevilFruit::new("Clone-Clone Fruit".to_string(), "Mane Mane no Mi".to_string()),
        DevilFruit::new("String-String Fruit".to_string(), "Ito Ito no Mi".to_string()),
        DevilFruit::new("Barrier-Barrier Fruit".to_string(), "Bari Bari no Mi".to_string()),
        DevilFruit::new("Op-Op Fruit".to_string(), "Ope Ope no Mi".to_string()),
        DevilFruit::new("Revive-Revive Fruit".to_string(), "Yomi Yomi no Mi".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
        DevilFruit::new("".to_string(), "".to_string()),
    ];

    let i = 0;
    while i < count
    {
        if language == "eng"
        {
            // get random devil fruit in english
        }
        else 
        {
            // get random devil fruit in japanese
        }
    }

}