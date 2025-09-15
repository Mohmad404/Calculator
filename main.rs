use std::io::*;

fn main() {
    println!("This just a calculator");

    loop {
        println!("enter the the operation NORMAL/AREA/UNITS/EXIT");
        let mut opreation = String::new();
        stdin().read_line(&mut opreation).expect("error");
        let opreation = opreation.trim();

        if opreation == "NORMAL" {
            println!("enter the opreater you want to use + or - or * or / or ^ or √ ");

            let mut opreater = String::new();
            stdin().read_line(&mut opreater).expect("error");
            let opreater = opreater.trim();

            if opreater == "+" {
                print!("enetr the first number: ");
                stdout().flush().expect("error");

                let mut num1 = String::new();
                stdin().read_line(&mut num1).expect("error");
                let num1: u64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                print!("enetr the secound number: ");
                stdout().flush().expect("error");

                let mut num2 = String::new();
                stdin().read_line(&mut num2).expect("error");
                let num2: u64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin1");
                        continue;
                    }
                };
                let eq1 = num1 + num2;
                println!("{eq1}");
            } else if opreater == "-" {
                print!("enetr the first number: ");
                stdout().flush().expect("error");

                let mut num1 = String::new();
                stdin().read_line(&mut num1).expect("error");
                let num1: u64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                print!("enetr the secound number: ");
                stdout().flush().expect("error");

                let mut num2 = String::new();
                stdin().read_line(&mut num2).expect("error");
                let num2: u64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin1");
                        continue;
                    }
                };
                let eq2 = num1 - num2;
                println!("{eq2}");
            } else if opreater == "*" {
                print!("enetr the first number: ");
                stdout().flush().expect("error");

                let mut num1 = String::new();
                stdin().read_line(&mut num1).expect("error");
                let num1: u64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                print!("enetr the secound number: ");
                stdout().flush().expect("error");

                let mut num2 = String::new();
                stdin().read_line(&mut num2).expect("error");
                let num2: u64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin1");
                        continue;
                    }
                };
                let eq3 = num1 * num2;
                println!("{eq3}");
            } else if opreater == "/" {
                print!("enetr the first number: ");
                stdout().flush().expect("error");

                let mut num1 = String::new();
                stdin().read_line(&mut num1).expect("error");
                let num1: u64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                print!("enetr the secound number: ");
                stdout().flush().expect("error");

                let mut num2 = String::new();
                stdin().read_line(&mut num2).expect("error");
                let num2: u64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin1");
                        continue;
                    }
                };
                let eq4 = num1 / num2;
                println!("{eq4}");
            } else if opreater == "^" {
                print!("enetr the number: ");
                stdout().flush().expect("error");

                let mut num1 = String::new();
                stdin().read_line(&mut num1).expect("error");
                let num1: f64 = match num1.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                print!("enetr the pow: ");
                stdout().flush().expect("error");

                let mut num2 = String::new();
                stdin().read_line(&mut num2).expect("error");
                let num2: f64 = match num2.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin1");
                        continue;
                    }
                };
                let eq5 = num1.powf(num2);
                println!("{eq5}");
            } else if opreater == "√" {
                print!("enter the number: ");
                stdout().flush().expect("error");

                let mut num = String::new();
                stdin().read_line(&mut num).expect("error");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                let eq6 = f64::sqrt(num);
                println!("{eq6}");
            } else {
                println!("invalid opreater please try agin!");
                continue;
            }
        } else if opreation == "AREA" {
            println!("enter the shape you want to calculat it aure Square/Triangle/Circle ");

            print!("enetr the shape: ");
            stdout().flush().expect("error");

            let mut shape = String::new();
            stdin().read_line(&mut shape).expect("error");
            let shape = shape.trim();

            if shape == "Square" {
                print!("enetr the length: ");
                stdout().flush().expect("error");

                let mut length = String::new();
                stdin().read_line(&mut length).expect("error");
                let length: i64 = match length.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please , Try agin!");
                        continue;
                    }
                };
                print!("enetr the height: ");
                stdout().flush().expect("error");

                let mut height = String::new();
                stdin().read_line(&mut height).expect("error");
                let height: i64 = match height.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please , Try agin!");
                        continue;
                    }
                };
                let eq6 = height * length;
                println!("the Square area is {}", eq6);
            } else if shape == "Triangle" {
                print!("enetr the length: ");
                stdout().flush().expect("error");

                let mut length = String::new();
                stdin().read_line(&mut length).expect("error");
                let length: f64 = match length.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                print!("enetr the height: ");
                stdout().flush().expect("error");

                let mut height = String::new();
                stdin().read_line(&mut height).expect("error");
                let height: f64 = match height.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                let eq7 = 0.5 * height as f64 * length as f64;
                println!("the Triangle area is {}", eq7);
            } else if shape == "Circle" {
                const PI: f64 = 3.14159;
                print!("enter the reduce: ");
                stdout().flush().expect("error");

                let mut reduce = String::new();
                stdin().read_line(&mut reduce).expect("error");
                let reduce: f64 = match reduce.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                let eq8 = PI * reduce.powf(2.0);
                println!("the Circle area is {}", eq8);
            }
        } else if opreation == "EXIT" {
            println!("bye");
            break;
        } else if opreation == "UNITS" {
            println!("enetr the Unit Circle you wanna use sin/cos/tan/sin-1/cos-1/tan-1 ");

            let mut unit = String::new();
            stdin().read_line(&mut unit).expect("error");
            let unit = unit.trim();

            if unit == "sin" {
                print!("enetr the namber you want to use: ");
                stdout().flush().expect("error");

                let mut sine = String::new();
                stdin().read_line(&mut sine).expect("error");
                let sine: f64 = match sine.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                let eq9 = sine.sin();
                println!("the sine is {}", eq9);
            } else if unit == "cos" {
                print!("enter the number you want to use: ");
                stdout().flush().expect("error");

                let mut cos = String::new();
                stdin().read_line(&mut cos).expect("error");
                let cos: f64 = match cos.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                let eq10 = cos.cos();
                println!("the cos is {}", eq10);
            } else if unit == "tan" {
                print!("enter the number you want to use: ");
                stdout().flush().expect("error");

                let mut tan = String::new();
                stdin().read_line(&mut tan).expect("error");
                let tan: f64 = match tan.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please Try agin!");
                        continue;
                    }
                };
                let eq11 = tan.tan();
                println!("the tan is {}", eq11);
            } else if unit == "sin-1" {
                print!("enter the number you want to use: ");
                stdout().flush().expect("error");

                let mut num = String::new();
                stdin().read_line(&mut num).expect("error");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please try agin!");
                        continue;
                    }
                };
                let eq12 = num.asin();
                println!("the sin-1 is {}", eq12);
            } else if unit == "cos-1" {
                print!("enter the number you want to use: ");
                stdout().flush().expect("error");

                let mut num = String::new();
                stdin().read_line(&mut num).expect("error");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please try agin!");
                        continue;
                    }
                };
                let eq13 = num.acos();
                println!("the cos-1 is {}", eq13)
            } else if unit == "tan-1" {
                print!("enter the number you want to use: ");
                stdout().flush().expect("error");

                let mut num = String::new();
                stdin().read_line(&mut num).expect("error");
                let num: f64 = match num.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("invalid input please try agin!");
                        continue;
                    }
                };
                let eq14 = num.atan();
                println!("the tan-1 is {}", eq14);
            }
        }
    }
}
