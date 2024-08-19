fn main(){
    let mut x: i8 = -11; // signed integer can be positive or negative
    let y: u8 = 99; // unsigned integer can only be positive
    let z: f32 = -4444442312124125125190896.; // float integer, cabe be +ve or -ve
    println!("x: {x}, y: {y}, z: {z}");
    println!("x: {}, y: {}, z: {}",x,y,z);


    // the for loop will compile the code but panic it on runtime
    // for _i in 0..100 {
        x = x+100; 
    // } 
    
    println!("x = {}",x);
    
    let greeting : String = String::from("Hello World!");
    println!("{}",greeting);

    let n:usize = 10;
    let char1: Option<char> = greeting.chars().nth(n);

    let sentence: String = String::from("Ram Ram Bhai Sareya n");
    let first_word: String = get_first_word(sentence);

    match char1{
        Some(c)=>print!("character at index {} of string '{}' is '{}'",n,greeting,c),
        None => println!("No character at index {}",n)
    };

    let nx: i32 = 1000;

    for r in 0..nx{
        println!("Hello world! {}",r)
    }

    println!("First word is '{}'", first_word);
}

fn get_first_word(sentence:String) -> String {
    let mut ans = String::from("");
    for char in sentence.chars(){
        ans.push_str(char.to_string().as_str());
        if char ==' ' {
            break;
        }
    }
    return ans;

}