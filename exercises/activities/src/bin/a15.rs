// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info
enum Ticket {
    Backstage(String, i32),
    Vip(String, i32),
    Standard(i32),
}

fn main() {
    let backstage_ticket = Ticket::Backstage(String::from("Alice"), 200);
    let vip_ticket = Ticket::Vip(String::from("Bob"), 100);
    let standard_ticket = Ticket::Standard(50);

    let tickets = vec![backstage_ticket, vip_ticket, standard_ticket];
    
         for ticket in tickets {
            match ticket {
                Ticket::Backstage(holder, price) => println!("Backstage ticket of {}, the price is {:?}", holder, price),
                Ticket::Vip(holder, price) => println!("Backstage ticket of {}, the price is {:?}", holder, price),
                Ticket::Standard(price) => println!("This is a standard ticket, the price is {:?}", price),
            }
         }
}
