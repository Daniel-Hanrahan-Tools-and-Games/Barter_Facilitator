use std::io;
use std::process;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::Row;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    // Necessary variables
    let dblConstHighestValue: f32 = 1.0;
    let mut dblUsersWant: f32;
    let mut dblAmtPeopleWhoHaveIt: f32;
    let mut dblValueOfProduct: f32;

    // Database connection URL (your .onion address)
    // The database_url must be "mysql://username:password@onion_address_for_mysql:3306/Barter_Facilitator_Database"
    let database_url = "";
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Copyright and license notice
    println!("Copyright (C) 2025 Daniel Hanrahan Tools and Games");
    println!("This program is free software: you can redistribute it and/or modify it under the terms of the GNU");
    println!("General Public License as published by the Free Software Foundation, either version 3 of the License, or");
    println!("(at your option) any later version.");
    println!("This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without");
    println!("even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.");
    println!("See the GNU General Public License for more details.");
    println!("You should have received a copy of the GNU General Public License along with this program. If not, see");
    println!("<https://www.gnu.org/licenses/>.");

    loop {
        // Display all offers
        let rows = sqlx::query("SELECT * FROM TOffers")
            .fetch_all(&pool)
            .await?;

        for row in rows {
            let intRecordID: i32 = row.get("intRecordID");
            let strEmail: String = row.get("strEmail");
            let strWants: String = row.get("strWants");
            let strHas: String = row.get("strHas");
            println!("ID: {}, Email: {}, Wants: {}, Has: {}", intRecordID, strEmail, strWants, strHas);
        }

        // Warnings
        println!("WARNING: Every trade must be negotiated by each party over email.");
        println!("WARNING: When using this software, you should use free and open source email services that cares about privacy.");
        println!("WARNING: Tax laws vary from area to area, keep that in mind when making a trade.");
        println!("WARNING: For privacy reasons, this application connects to database over TOR, so it may be slow.");
        println!("WARNING: To exit application press any key then enter that is not an option");

        // Menu prompt
        println!("Do you want to add offer, yes = 1 or no = 2: ");
        let mut strMenu = String::new();
        io::stdin().read_line(&mut strMenu).unwrap();

        match strMenu.trim().parse::<i32>() {
            Ok(intMenu) => {
                if intMenu == 1 {
                    // Add offer
                    let mut strEmailInput = String::new();
                    let mut strWhatUserWantsInput = String::new();
                    let mut strWhatUserHasInput = String::new();

                    println!("What is your Email: ");
                    io::stdin().read_line(&mut strEmailInput).expect("failed to readline");
                    println!("What do you want: ");
                    io::stdin().read_line(&mut strWhatUserWantsInput).expect("failed to readline");
                    println!("What are you willing to trade for it: ");
                    io::stdin().read_line(&mut strWhatUserHasInput).expect("failed to readline");

                    // Trim input and convert to uppercase
                    let strEmailInputUpper = strEmailInput.trim().to_uppercase();
                    let strWhatUserWantsInputUpper = strWhatUserWantsInput.trim().to_uppercase();
                    let strWhatUserHasInputUpper = strWhatUserHasInput.trim().to_uppercase();

                    // Insert into database
                    sqlx::query("INSERT INTO TOffers (strEmail, strWants, strHas) VALUES (?, ?, ?)")
                        .bind(&strEmailInputUpper)
                        .bind(&strWhatUserWantsInputUpper)
                        .bind(&strWhatUserHasInputUpper)
                        .execute(&pool)
                        .await?;

                    println!("Inserted Record: Email = {}, Wants = {}, Has = {}", strEmailInputUpper, strWhatUserWantsInputUpper, strWhatUserHasInputUpper);

                    // Count how many people who have the item user wants
                    let AmtPeopleWhoHasItRow: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM TOffers WHERE strHas = ?")
                        .bind(&strWhatUserWantsInputUpper)
                        .fetch_one(&pool)
                        .await?;

                    dblAmtPeopleWhoHaveIt = AmtPeopleWhoHasItRow.0 as f32;

                    // Count how many other people want the item user wants
                    let AmtPeopleWhoWantsItRow: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM TOffers WHERE strWants = ?")
                        .bind(&strWhatUserWantsInputUpper)
                        .fetch_one(&pool)
                        .await?;

                    dblUsersWant = AmtPeopleWhoWantsItRow.0 as f32;

                    // Calculate value of product
                    // To avoid division by zero or negative, add check
                    let denominator = dblAmtPeopleWhoHaveIt - dblUsersWant;
                    if denominator <= 0.0 {
                        println!("The value of {} is 1 because it is the only offer in here that has this item.", strWhatUserWantsInputUpper);
                    } else {
                        dblValueOfProduct = dblConstHighestValue / denominator;
                        println!("The value of {} is now {}", strWhatUserWantsInputUpper, dblValueOfProduct);
                    }

                } else if intMenu == 2 {
                    // User chose not to add offer
                    println!("You Decided not to add offer");

                    // Delete prompt
                    println!("Do you want to delete offer, yes = 1 or no = 2: ");
                    let mut strDeletePrompt = String::new();
                    io::stdin().read_line(&mut strDeletePrompt).unwrap();

                    match strDeletePrompt.trim().parse::<i32>() {
                        Ok(intDeletePrompt) => {
                            if intDeletePrompt == 1 {
                                // Delete offer
                                println!("Please type in the ID number of the record you want to delete: ");
                                let mut strWhatRecordToDelete = String::new();
                                io::stdin().read_line(&mut strWhatRecordToDelete).unwrap();

                                match strWhatRecordToDelete.trim().parse::<i32>() {
                                    Ok(intWhatRecordToDelete) => {
                                        sqlx::query("DELETE FROM TOffers WHERE intRecordID = ?")
                                            .bind(intWhatRecordToDelete)
                                            .execute(&pool)
                                            .await?;

                                        println!("Deleted record {}", intWhatRecordToDelete);
                                        println!("You will want to check on prices now");
                                    }
                                    Err(_) => {
                                        println!("Not a Number, GoodBye");
                                        process::exit(0);
                                    }
                                }
                            } else if intDeletePrompt == 2 {
                                // User chose not to delete
                                println!("You Decided not to delete offer");

                                // Search prompt
                                println!("Do you want to search for offers of a specific item or find the value of something, yes = 1 or no = 2: ");
                                let mut strSearchPrompt = String::new();
                                io::stdin().read_line(&mut strSearchPrompt).unwrap();

                                match strSearchPrompt.trim().parse::<i32>() {
                                    Ok(intSearchPrompt) => {
                                        if intSearchPrompt == 1 {
                                            // Search offers
                                            println!("What do you want to search for or find the value of: ");
                                            let mut strWhatUserWantsInput = String::new();
                                            io::stdin().read_line(&mut strWhatUserWantsInput).expect("failed to readline");
                                            let strWhatUserWantsInputUpper = strWhatUserWantsInput.trim().to_uppercase();

                                            // Query offers where strHas = ?
                                            let rows = sqlx::query("SELECT * FROM TOffers WHERE strHas = ?")
                                                .bind(&strWhatUserWantsInputUpper)
                                                .fetch_all(&pool)
                                                .await?;

                                            for row in rows {
                                                let intRecordIDValueCheck: i32 = row.get("intRecordID");
                                                let strEmailValueCheck: String = row.get("strEmail");
                                                let strWantsValueCheck: String = row.get("strWants");
                                                let strHasValueCheck: String = row.get("strHas");
                                                println!("ID: {}, Email: {}, Wants: {}, Has: {}", intRecordIDValueCheck, strEmailValueCheck, strWantsValueCheck, strHasValueCheck);
                                            }

                                            // Count who has the item
                                            let AmtPeopleWhoHasItRow: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM TOffers WHERE strHas = ?")
                                                .bind(&strWhatUserWantsInputUpper)
                                                .fetch_one(&pool)
                                                .await?;

                                            dblAmtPeopleWhoHaveIt = AmtPeopleWhoHasItRow.0 as f32;

                                            // Count who wants the item
                                            let AmtPeopleWhoWantsItRow: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM TOffers WHERE strWants = ?")
                                                .bind(&strWhatUserWantsInputUpper)
                                                .fetch_one(&pool)
                                                .await?;

                                            dblUsersWant = AmtPeopleWhoWantsItRow.0 as f32;

                                            let denominator = dblAmtPeopleWhoHaveIt - dblUsersWant;
                                            if denominator <= 0.0 {
                                                println!("The value of {} is 1 because it is the only offer in here that has this item.", strWhatUserWantsInputUpper);
                                            } else {
                                                dblValueOfProduct = dblConstHighestValue / denominator;
                                                println!("The value of {} is {}", strWhatUserWantsInputUpper, dblValueOfProduct);
                                            }
                                        } else if intSearchPrompt == 2 {
                                            println!("You Decided not to search for offers or find the value.");
                                        } else {
                                            println!("Not An Option, GoodBye");
                                            process::exit(0);
                                        }
                                    }
                                    Err(_) => {
                                        println!("Not a Number, GoodBye");
                                        process::exit(0);
                                    }
                                }
                            } else {
                                println!("Not An Option, GoodBye");
                                process::exit(0);
                            }
                        }
                        Err(_) => {
                            println!("Not a Number, GoodBye");
                            process::exit(0);
                        }
                    }
                } else {
                    println!("Not An Option, GoodBye");
                    process::exit(0);
                }
            }
            Err(_) => {
                println!("Not a Number, GoodBye");
                process::exit(0);
            }
        }
    }
}

