use std::{fs::{self}, io::Write, path::Path};

fn main() {
    let mut width_input = String::new();
    let mut height_input = String::new();

    print!("Width for image: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut width_input).expect("Error while taking input");

    print!("Height for image: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut height_input).expect("Error while taking input");

    
    let width = width_input.trim().parse::<i32>().expect("Error while converting to i32 in width variable");
    let height = height_input.trim().parse::<i32>().expect("Error while converting to i32 in height variable");

    let mut my_matrix = vec![];

    for row in 0..height{
        let mut row_vec = vec![];
        for column in 0..width {
            print!("Value for {row}x{column}: ");
            std::io::stdout().flush().unwrap();
            let mut value_buffer = String::new();
            std::io::stdin().read_line(&mut value_buffer).expect("Error when read stdin");
            let value = value_buffer.trim().parse::<i32>().expect("Error when parse stdin to value (value variable)");
            row_vec.push(value);
        }
        my_matrix.push(row_vec);
    }

    // show vector
    for row in &my_matrix {
        for column in row {
            print!("{column}\t")
        }
        print!("\n");
    }
    let path = Path::new("image_result/test.pbm");
    let mut test_file = fs::File::create(path).expect("Error when creating file");
    test_file.write(b"P1\n").unwrap();
    test_file.write(format!("{width} {height}").as_bytes()).unwrap();
    test_file.write(b"\n");
    
    for row in my_matrix{
        for column in row {
            test_file.write(column.to_string().as_bytes()).unwrap();
            
            //stop tab if the column is the last elemen 
            if column != width - 1 {
                test_file.write(b"\t").unwrap();
            }
        }
        test_file.write("\n".as_bytes()).unwrap();
    }
}
