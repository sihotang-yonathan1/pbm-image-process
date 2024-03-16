use std::{fs, io::{self, Write}, path::Path};

fn input(message: &str) -> String {
    print!("{message}");
    std::io::stdout().flush().unwrap(); // flush after println!

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Error when readline input");
    buffer
}

fn main() {
    let file_extension = input("file extension (pbm, pgm): ");
    let filename = input("file name (just name): ");
    let width_input = input("Width for image: ");
    let height_input = input("Height for image: ");

    
    let width = width_input.trim().parse::<i32>().expect("Error while converting to i32 in width variable");
    let height = height_input.trim().parse::<i32>().expect("Error while converting to i32 in height variable");

    let mut my_matrix = vec![];

    for row in 0..height{
        let mut row_vec = vec![];
        for column in 0..width {
            let value_buffer = input(format!("Value for {row}x{column}: ").as_str());
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

    let new_filename = &filename.trim().to_string();
    let new_file_extension = &file_extension.clone().trim().to_string();

    let new_file_path = format!("image_result/{new_filename}.{new_file_extension}");
    let path = Path::new(new_file_path.as_str());
    let mut test_file = fs::File::create(path).expect("Error when creating file");
    test_file.write(b"P1\n").unwrap();
    test_file.write(format!("{width} {height}").as_bytes()).unwrap();
    test_file.write(b"\n").unwrap();
    
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
