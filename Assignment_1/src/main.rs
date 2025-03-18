fn main() {
    // With funtioin
    println!("Everything about me:\n");
    about(32.0,7.0);
    println!("\n");

    // Without function
    println!("My name is Eva, \nMat number: ENG2102751");
    let newemoji:char= '\u{1F602}';
    println!("My favourite emoji is {}", newemoji);
    let a:f32= 32.0;
    let b:f32 = 6.0;
    println!("When you divide {} and {}, you get: {}", a,b,a/b);
}

fn about(a:f32, b:f32){
    println!("Hi my name is Eva,\nMat number: ENG2102751.");
    let emojicharacter:char= '\u{1F602}';
    println!("My favourite emoji is the {} emoji",emojicharacter);
    println!("When you divide {} and {}, you get: {}", a,b,a/b);
}



 