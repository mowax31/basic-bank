use std::io;

fn intro() {

        let usr = "im.mowthik";
        let pas = "sinkara123";

        let us = io::stdin();
        let mut ussr = String::new();
        let mut pass = String::new();

        println!("Welcome to Mowax Bank");

        println!("Username: {}", ussr);
        us.read_line(&mut ussr).expect("can't read username");

        println!("Password: {}", pass);
        us.read_line(&mut pass).expect("can't read Password");

        if ussr.trim() == usr && pass.trim() == pas {
                pinz();
        } else {
                println!("Password in correct");
        }

}


fn pinz() {

        let pinx = io::stdin();
        let pinno: i32 = 1258;
        let mut pin = String::new();

        println!("Welcome the PIN ID:");

        println!("Enter the Pin no -> {} ", pin);
        pinx.read_line(&mut pin).unwrap();
        let pin: i32 = pin.trim().parse().unwrap();

        if pin == pinno {
                fir();
        } else {
                println!("Incorrect PIN");
        }
}

fn fir() {

        let mut ip = String::new();
        println!("Enter the number by the choice");

        println!("1. Savings");
        println!("2. current");
        println!("3. Bank Balance");
        println!("4. exit");

        io::stdin().read_line(&mut ip).unwrap();

        let sw: u32 = ip.trim().parse().unwrap();
        match sw {
                1 => sav(),
                2 => cur(),
                3 => bb(),
                4 => exit(),
                _ => {
        println!("valid choice");
        fir();
        },
        }
}


fn sav() {
        let mut money = String::new();
        println!("Enter the Amount {}", money);
        io::stdin().read_line(&mut money).unwrap();
        let money: u32 = money.trim().parse().unwrap();
        println!("Successfully Transfered {} from savings account", money);
        let mut cr = String::new();
        println!("like To go Back? press Y/n ...");
        io::stdin().read_line(&mut cr).expect("cant read");
        let cr: char = cr.trim().parse().expect("can't read");
        println!("Y/N {}", cr);
        if cr == 'y' {
                fir();
        } else {
                println!("OK ");
        }
}

fn cur() {
        let mut money = String::new();
        println!("Enter the Amount {}", money);
        io::stdin().read_line(&mut money).unwrap();
        let money: u32 = money.trim().parse().unwrap();
        println!("Successfully Transfered {} from current account", money);
        let mut cr = String::new();
        println!("like To go Back? press Y/n ...");
        io::stdin().read_line(&mut cr).expect("cant read");
        let cr: char = cr.trim().parse().expect("can't read");
        println!("Y/N {}", cr);
        if cr == 'y' {
                fir();
        } else {
                println!("OK ");
        }
}

fn bb() {
        let bank = 10000.00;
        println!("your Bank Balance is {}", bank);
        let mut cr = String::new();
        println!("like To go Back? press Y/n ...");
        io::stdin().read_line(&mut cr).expect("cant read");
        let cr: char = cr.trim().parse().expect("can't read");
        println!("Y/N {}", cr);
        if cr == 'y' {
                fir();
        } else if cr == 'n' {
                println!("OK ");
        } else {
                println!("type only Y or N");
        }
}

fn exit() {
        let mut e = String::new();
        println!("Do you really wanna exit ?");
        io::stdin().read_line(&mut e).expect("cant read e");
        let e: char = e.trim().parse().expect("cant read ");
        println!("y/n {}", e);
        if e == 'y' {
                println!("Thank you for Visiting, Bye");
        } else if e == 'n' {
                println!("wanna go back? to choise");
                if e == 'y' {
                        fir();
                } else {
                        println!("OK");
                }
        } else {
                println!("type only Y / N");
        }


}

fn main() {
        intro();
}
