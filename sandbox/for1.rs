fn main() {
    for i in 0..31 {
        // println!("i = {}", i);
        let fiz = if i % 3 == 0 {"fiz".to_string()} else {"".to_string()};
        let buz = if i %5 == 0 {"buz".to_string()} else {"".to_string()};
        let line = if fiz != "" || buz != "" {fiz + &buz} else {i.to_string()};
        println!("{}", line);
    }

    let mut sum = 0;
    for i in 0..5 {
        sum += i;
    }
    println!("sum {}", sum);

    line('A', 'D')
}

// fn diamond(c: char) {
//     for i in 'A' .. c {
//         line(i, c);
//     }
// }
// fn line(c: char, end: char) {

//     let spaces = end as u32 - c as u32;
//     let chars = (c as u32 - 'A' as u32) * 2 + 1;
//     println!("{}{}", " ".repeat(spaces as usize), c.to_string().repeat(chars as usize));

// }
fn line(c: char, end: char) {

    if c > end {
        return;
    }

    let spaces = end as u32 - c as u32;
    let chars = (c as u32 - 'A' as u32) * 2 + 1;
    let message = " ".repeat(spaces as usize).to_string() +  &c.to_string().repeat(chars as usize).to_string();
    println!("{}", message);

    line(((c as u8)+1) as char, end);
    if c != end {
        println!("{}", message);
    }
}