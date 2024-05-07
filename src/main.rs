use clap::{Arg, Command};
use rand::Rng;

// today we are doing argument parsing with Clap

// create devil fruit struct
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct DevilFruit
{
    english_name: String,
    japanese_name: String
}

// implement devil fruit struct
impl DevilFruit
{
    // create new function to create instances of the struct
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
    // create arguments and info
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
        .arg(Arg::new("noduplicates")
                .short('n')
                .long("nodup")
                .action(clap::ArgAction::SetTrue)
                .help("if present, removes duplicates from list"))
        .get_matches();

    // get values
    let default_lang = "eng".to_string();
    let language = matches.get_one::<String>("lang").unwrap_or(&default_lang);
    let count_str = matches.get_one::<String>("count");
    let no_dupe = matches.get_flag("noduplicates");

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
        DevilFruit::new("Flower-Flower Fruit".to_string(), "Hana Hana no Mi".to_string()),
        DevilFruit::new("Human-Human Fruit".to_string(), "Hito Hito no Mi".to_string()),
        DevilFruit::new("Dark-Dark Fruit".to_string(), "Yami Yami no Mi".to_string()),
        DevilFruit::new("Ice-Ice Fruit".to_string(), "Hie Hie no Mi".to_string()),
        DevilFruit::new("Plume-Plume Fruit".to_string(), "Moku Moku no Mi".to_string()),
        DevilFruit::new("Gum-Gum Fruit".to_string(), "Gomu Gomu no Mi".to_string()),
        DevilFruit::new("Chop-Chop Fruit".to_string(), "Bara Bara no Mi".to_string()),
        DevilFruit::new("Horm-Horm Fruit".to_string(), "Horu Horu no Mi".to_string()),
        DevilFruit::new("Sand-Sand Fruit".to_string(), "Suna Suna no Mi".to_string()),
        DevilFruit::new("Love-Love Fruit".to_string(), "Mero Mero no Mi".to_string()),
        DevilFruit::new("Shadow-Shadow Fruit".to_string(), "Kage Kage no Mi".to_string())
    ];

    // for each count
    let mut i = 0;
    while i < count
    {
        // if out of fruits because no duplicated items
        if fruits.len() == 0
        {
            println!("uh oh! ran out of devil fruits!\ntry running without --nodup!");
            return;
        }

        // create rng based on thread
        let mut rng = rand::thread_rng();

        // generate random number based on rng and range
        let index = rng.gen_range(0..fruits.len());

        // current fruit
        let fruit = &fruits[index];

        // print depending on language
        if language == "eng"
        {
            println!("{}", fruit.english_name);
        }
        else 
        {
            println!("{}", fruit.japanese_name);
        }
        
        // iterate
        i = i + 1;

        // if no dupe, remove current fruit from list
        if no_dupe
        {
            fruits.remove(index);
        }
    }

}