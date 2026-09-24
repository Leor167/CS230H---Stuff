fn main() {
    //My friends username
    let mut user = String::from("AnimeTutle");
    //print username original
    println!("user is {user}");
    //print username after it borrowed a mutable reference for user and altered the original.
    change_username(&mut user);
    user = process_user(user);
    
    let mmr = 123;
    let rank = process_mmr(mmr);
    println!("{user} rank is {rank}");
}
//takes ownership to use in process user then returns it
fn process_user(user_id: String) -> String {

    println!("processing name: {user_id}");
    user_id
} 




//borrows user_id from reference so the function can alter the original string.
fn change_username(user_id : &mut String){
    user_id.push_str("#NA");
    println!("Updated name: {user_id}");
}


//i32 is a scalar variables so you can just pass a value straight to it without worrying about ownership since i32 uses copy
//which makes a copy of value in mmr to be used.
fn process_mmr(mmr : i32) -> String {
    println!("Processing {mmr}");
    //I had () on the if conditionals but apparently im not supposed to have that
    if mmr < 100{
        //in rust you can have an expression without a semicolon to use as a return value
        String::from("bronze")
    }
    else if mmr < 200{
        String::from("silver")
    }
    else {
        String::from("gold")
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string_takes_ownership(){
        let user = String::from("Firelaser");
        process_user(user); //not giving it back to user by let user = process_user(user)
        //println!("Do I still own {user}?"); <- didn't compile since it didn't own the value anymore
    }
    #[test]
    fn test_string_returns_ownership(){
        let mut user = String::from("Firelaser");
        user = process_user(user); //gives back ownership
        println!("Do I still own {user}?"); //compiles
    }
    #[test]
    fn test_i32_copy(){
        let mmr = 100;
        process_mmr(mmr);
        println!("Do I still own {mmr}?"); // compiled and printed
    }
}