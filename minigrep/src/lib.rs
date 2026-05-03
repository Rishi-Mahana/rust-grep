pub fn search<'a>(query:&str, contents: &'a str)->Vec<&'a str>{
    let mut output=Vec::new();
    for line in contents.lines(){
        if line.contains(&query){
            output.push(line);
        }
    }
    return output;
}
pub fn search_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str>{
    let query=query.to_lowercase();
    let mut output = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query){
            output.push(line);
        }
    }
    return output;
}

