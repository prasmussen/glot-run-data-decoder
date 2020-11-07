use std::fs;
use std::env;
use std::collections::HashMap;


enum FileType {
    Users(),
    Languages(),
}


fn main() {
    let args = env::args()
        .collect::<Vec<String>>();

    let args_str = args.iter()
        .map(|s| s.as_ref())
        .collect::<Vec<&str>>();

    match &args_str[1..] {
        ["users", fname] => {
            decode(FileType::Users(), fname)
        }

        ["languages", fname] => {
            decode(FileType::Languages(), fname)
        }

        _ => {
            println!("Usage:\n{} users <users.dat>\n{} languages <languages.dat>", args[0], args[0]);
        }
    }
}

fn decode(file_type: FileType, fname: &str) {
    let file = fs::File::open(fname).unwrap();
    let term = eetf::Term::decode(file).unwrap();

    match file_type {
        FileType::Users() => {
            let hashmap = decode_users(term);
            write_json_output(hashmap)
        }

        FileType::Languages() => {
            let hashmap = decode_languages(term);
            write_json_output(hashmap)
        }
    }
}

fn write_json_output<T: serde::Serialize>(hashmap: HashMap<String, T>) {
    let json = serde_json::to_string_pretty(&hashmap).unwrap();
    println!("{}", json);
}


#[derive(Debug, Clone, serde::Serialize)]
pub struct Language {
    pub id: String,
    pub name: String,
    pub version: String,
    pub image: String,
}

fn decode_languages(term: eetf::Term) -> HashMap<String, Language> {
    get_map(&term).entries.iter().map(|(key, value)| {
        let id = get_binary_string(key);
        let tuple = get_tuple(value);
        (id.clone(), to_language(&id, &tuple))
    }).collect()
}

fn to_language(id: &str, tuple: &[eetf::Term]) -> Language {
    match tuple {
        [name, version, image] => {
            Language{
                id: id.to_string(),
                name: get_binary_string(name),
                version: get_binary_string(version),
                image: get_binary_string(image),
            }
        }

        _ => {
            panic!("Expected 3-element tuple")
        }
    }
}



#[derive(Debug, Clone, serde::Serialize)]
pub struct User {
    pub id: String,
    pub token: String,
    pub created: String,
    pub modified: String,
}

fn decode_users(term: eetf::Term) -> HashMap<String, User> {
    get_map(&term).entries.iter().map(|(key, value)| {
        let id = get_binary_string(key);
        let user_map = get_hashmap(value);
        (id, to_user(user_map))
    }).collect()
}

fn to_user(data: HashMap<String, String>) -> User {
    User{
        id: data.get("id").unwrap().clone(),
        token: data.get("token").unwrap().clone(),
        created: data.get("created").unwrap().clone(),
        modified: data.get("modified").unwrap().clone(),
    }
}


fn get_hashmap(term: &eetf::Term) -> HashMap<String, String> {
    get_map(term).entries.iter().map(|(key, value)| {
        (get_atom_string(key), get_binary_string(value))
    }).collect()
}

fn get_map(term: &eetf::Term) -> eetf::Map {
    match term {
        eetf::Term::Map(map) => {
            map.clone()
        }

        _ => {
            panic!("Expected map, got: {:?}", term)
        }
    }
}

fn get_atom_string(term: &eetf::Term) -> String {
    match term {
        eetf::Term::Atom(s) => {
            s.name.clone()
        }

        _ => {
            panic!("Expected atom, got: {:?}", term)
        }
    }
}

fn get_binary_string(term: &eetf::Term) -> String {
    match term {
        eetf::Term::Binary(binary) => {
            String::from_utf8(binary.bytes.clone()).unwrap()
        }

        _ => {
            panic!("Expected binary, got: {:?}", term)
        }
    }
}

fn get_tuple(term: &eetf::Term) -> Vec<eetf::Term> {
    match term {
        eetf::Term::Tuple(tuple) => {
            tuple.elements.clone()
        }

        _ => {
            panic!("Expected tuple, got: {:?}", term)
        }
    }
}
